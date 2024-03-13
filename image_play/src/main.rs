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

fn get_url_from_args() -> String {
    let args = Args::parse();
    
    args
        .url
        .to_string()
}

#[tokio::main]
async fn main() -> Result<()> {
    let target = get_url_from_args();
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

