use error_chain::error_chain;
use std::io::{copy, Cursor};
use std::fs::File;
use clap::Parser;

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
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    println!("Will download image from: {}", args.url);
    let target = &args.url;

    //let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    //let target: &str = "https://ik.imagekit.io/theartling/prod/tr:w-1840,c-at_max/original_images/Sarah_Lee_Gesturebation.jpg";
    let response = reqwest::get(target).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let fname = format!("./{}", fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname)?
    };
    let mut content = Cursor::new(response.bytes().await?);
    copy(&mut content, &mut dest)?;
    Ok(())
}

