use tracing_subscriber::filter::{EnvFilter, LevelFilter};use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use aws_sdk_s3 as s3;
use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Config;
// use tokio_util::bytes::Bytes;
use aws_sdk_dynamodb as dynamodb;

use resize::Pixel::RGB8;
use resize::Type::Lanczos3;
use rgb::RGB8;
use rgb::FromSlice;

use serde::{Deserialize, Serialize};

// begin from https://crates.io/crates/fast_image_resize
use std::io::BufWriter;
use std::num::NonZeroU32;

use image::codecs::png::PngEncoder;
use image::io::Reader as ImageReader;
use image::{ColorType, ImageEncoder};
//use cursor::Cursor;
use std::io::Cursor;

use fast_image_resize::{Image, FilterType, resize};
//end
//
#[derive(Deserialize)]
struct Request {
    command: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

fn fast_image_resize() {
        // Read source image from file
    let img = ImageReader::open("./data/nasa-4928x3279.png")
        .unwrap()
        .decode()
        .unwrap();
    let width = NonZeroU32::new(img.width()).unwrap();
    let height = NonZeroU32::new(img.height()).unwrap();
    let mut src_image = fr::Image::from_vec_u8(
        width,
        height,
        img.to_rgba8().into_raw(),
        fr::PixelType::U8x4,
    ).unwrap();

    // Multiple RGB channels of source image by alpha channel 
    // (not required for the Nearest algorithm)
    let alpha_mul_div = fr::MulDiv::default();
    alpha_mul_div
        .multiply_alpha_inplace(&mut src_image.view_mut())
        .unwrap();

    // Create container for data of destination image
    let dst_width = NonZeroU32::new(1024).unwrap();
    let dst_height = NonZeroU32::new(768).unwrap();
    let mut dst_image = fr::Image::new(
        dst_width,
        dst_height,
        src_image.pixel_type(),
    );

    // Get mutable view of destination image data
    let mut dst_view = dst_image.view_mut();

    // Create Resizer instance and resize source image
    // into buffer of destination image
    let mut resizer = fr::Resizer::new(
        fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3),
    );
    resizer.resize(&src_image.view(), &mut dst_view).unwrap();

    // Divide RGB channels of destination image by alpha
    alpha_mul_div.divide_alpha_inplace(&mut dst_view).unwrap();

    // Write destination image as PNG-file
    let mut result_buf = BufWriter::new(Vec::new());
    PngEncoder::new(&mut result_buf)
        .write_image(
            dst_image.buffer(),
            dst_width.get(),
            dst_height.get(),
            ColorType::Rgba8,
        )
        .unwrap();
}

fn do_resize() -> Result<(), resize::Error>{
    let (w1, h1) = (640, 480);
    let (w2, h2) = (320, 240);
    // Don't forget to fill `src` with image data (RGB8).
    let src = vec![0;w1*h1*3];
    // Destination buffer. Must be mutable.
    let mut dst = vec![0;w2*h2*3];
    // Create reusable instance.
    let mut resizer = resize::new(w1, h1, w2, h2, RGB8, Lanczos3)?;
    // Do resize without heap allocations.
    // Might be executed multiple times for different `src` or `dst`.
    resizer.resize(src.as_rgb(), dst.as_rgb_mut());
    Ok(())
}

pub async fn list_objects(client: &Client, bucket: &str) -> Result<(), Error> {
    let mut response = client
        .list_objects_v2()
        .bucket(bucket.to_owned())
        .max_keys(10) // In this example, go 10 at a time.
        .into_paginator()
        .send();

    while let Some(result) = response.next().await {
        match result {
            Ok(output) => {
                for object in output.contents() {
                    println!(" - {}", object.key().unwrap_or("Unknown"));
                }
            }
            Err(err) => {
                eprintln!("{err:?}")
            }
        }
    }

    Ok(())
}

pub async fn put_file(
    client: &Client, 
    bucket: &str, 
    key: &str, 
    body: ByteStream
) -> Result<(), Error> {
    println!("Putting file");
    println!("Bucket: {}", bucket);
    println!("Key: {}", key);

    const BODY: &str = "Hello, world!";

    let x: ByteStream = ByteStream::from_static(BODY.as_bytes());


    let response = client
        .put_object()
        .bucket(bucket.to_owned())
        .key(key.to_owned())
        //.body(ByteStream::from_static(BODY.as_bytes()))
        .body(x)
        .send()
        .await?;

    println!("Response: {:?}", response);

    Ok(())
}

async fn handle_chunk(mut chunk: bytes::BytesMut) -> Result<(), Error> {
    // Decode the chunk using an image library
    let decoded_image = image::io::Reader::new(Cursor::new(&chunk))
        .with_guessed_format()?
        .decode()?;

    // Resize the decoded image
    let resized_image = resize(&mut decoded_image, 320, 240, FilterType::Lanczos3);

    // Upload the resized data to S3 (modify this section)
    // ...

    Ok(())
}

pub async fn get_file(client: &Client, bucket: &str, key: &str) -> Result<(), Error> {
    println!("Getting file");
    println!("Bucket: {}", bucket);
    println!("Key: {}", key);

    let response = client
        .get_object()
        .bucket(bucket.to_owned())
        .key(key.to_owned())
        .send()
        .await?;

    let body = response.body;
    // println!("Body: {:?}", body);

    // put_file(client, "dan-images-resized", "test.txt", body).await?;

    //println!("Data: {:?}", data);
    // try put body into a file
    // let new_key = "blah/test.txt";
    // put_file(client, bucket, new_key, data.chunk()).await?;



    Ok(())
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let file_name = event.payload.command;

    //talk to S3
    let config = aws_config::load_from_env().await;
    let s3_client = aws_sdk_s3::Client::new(&config);
    list_objects(&s3_client, "dan-images-resized").await?;

    println!("Getting file");
    get_file(&s3_client, "dan-images-resized", file_name.as_str()).await?;

    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("File name {}.", file_name),
    };

    do_resize().unwrap();
    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
