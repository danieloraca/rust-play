use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    profile: String,
    #[arg(short, long)]
    region: String,
    // #[arg(short, long)]
    // table: String,
}

pub fn parse_args() {
    let args = Args::parse();
    env::set_var("AWS_PROFILE", &args.profile);
    env::set_var("AWS_REGION", &args.region);
    // env::set_var("DYNAMODB_TABLE", &args.table);
}
