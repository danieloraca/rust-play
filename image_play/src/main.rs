use error_chain::error_chain;
use std::io::{copy, Cursor};
use std::fs::File;
use clap::Parser;

use image::codecs::png::PngEncoder;
use image::io::Reader as ImageReader;
use image::{ImageEncoder, ExtendedColorType};

use std::num::NonZeroU32;
use fast_image_resize::Image as fr_image;
use fast_image_resize::Resizer as fr_resizer;
use fast_image_resize::PixelType as fr_pixel_type;
use fast_image_resize::MulDiv as fr_mul_div;
use fast_image_resize::FilterType as fr_filter_type;
use fast_image_resize::ResizeAlg as fr_resize_alg;

use std::io::BufWriter;


error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    width: Option<u32>,
    url: String,
}

fn get_url_from_args() -> String {
    let args = Args::parse();
    args
        .url
        .to_string()
}

fn get_width_from_args() -> Option<u32> {
    let args = Args::parse();
    args
        .width
}

fn resize_image(arg_width: Option<u32>, fname: String) -> Result<()> {
    //println!("Resizing image: {}", fname);
    let img = ImageReader::open(fname)
        .unwrap()
        .decode()
        .unwrap();

    let width = NonZeroU32::new(img.width()).unwrap();
    let height = NonZeroU32::new(img.height()).unwrap();

    let mut src_image = fr_image::from_vec_u8(
        width,
        height,
        img.to_rgba8().into_raw(),
        fr_pixel_type::U8x4,
    ).unwrap();

    let alpha_mul_div = fr_mul_div::default();
    alpha_mul_div
        .multiply_alpha_inplace(&mut src_image.view_mut())
        .unwrap();

    let w = arg_width.unwrap_or(800);
    let h = (f64::from(w) * f64::from(height.get()) / f64::from(width.get())) as u32;

    // Create container for data of destination image
    let dst_width = NonZeroU32::new(w).unwrap();
    let dst_height = NonZeroU32::new(h).unwrap();
    let mut dst_image = fr_image::new(
        dst_width,
        dst_height,
        src_image.pixel_type(),
    );

    // Get mutable view of destination image data
    let mut dst_view = dst_image.view_mut();

    // Create Resizer instance and resize source image
    // into buffer of destination image
    let mut resizer = fr_resizer::new(
        fr_resize_alg::Convolution(fr_filter_type::Lanczos3),
    );
    resizer.resize(&src_image.view(), &mut dst_view).unwrap();

    // Divide RGB channels of destination image by alpha
    alpha_mul_div.divide_alpha_inplace(&mut dst_view).unwrap();

    println!("Resized image: {}x{}", dst_width, dst_height);
    // Write destination image as PNG-file
    let mut result_buf = BufWriter::new(Vec::new());
    PngEncoder::new(&mut result_buf)
        .write_image(
            dst_image.buffer(),
            dst_width.get(),
            dst_height.get(),
            ExtendedColorType::Rgba8,
        )
        .unwrap();

    let result = result_buf.into_inner().unwrap();
    let mut file = File::create("resized.png")?;
    copy(&mut Cursor::new(result), &mut file)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let target = get_url_from_args();
    let width = get_width_from_args();

    let response = reqwest::get(target).await?;
    let file_name: String;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        //println!("file to download: '{}'", &fname);
        let fname = format!("./{}", fname);
        file_name = fname.clone();
        //println!("will be located under: '{:?}'", fname);
        File::create(fname)?
    };
    let mut content = Cursor::new(response.bytes().await?);
    copy(&mut content, &mut dest)?;

    resize_image(width, file_name)?;

    Ok(())
}

