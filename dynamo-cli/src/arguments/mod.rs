use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    profile: String,
}

pub fn parse_args() {
    let args = Args::parse();
    env::set_var("AWS_PROFILE", &args.profile);
    println!("AWS_PROFILE: {}", args.profile);
}
