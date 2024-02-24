use tracing_subscriber::filter::{EnvFilter, LevelFilter};use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use aws_sdk_s3 as s3;
use aws_sdk_s3::Client;
use aws_sdk_s3::Config;

use aws_sdk_dynamodb as dynamodb;

use resize::Pixel::RGB8;
use resize::Type::Lanczos3;
use rgb::RGB8;
use rgb::FromSlice;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    command: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
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
    println!("Body: {:?}", body);
//
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
