mod configuration;

use std::env;
use clap::Parser;





#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String
}


fn main() {
    
    let args = Args::parse();
    println!("yay {}!", args.name);
}

