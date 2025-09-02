mod core;

use crate::core::types::{FileSection, Line};
use clap::Parser;
use smol::{fs, prelude::*};
use std::{path::PathBuf};

#[derive(Debug, Parser)]
#[command(name = "mdcon")]
enum Cli {
    Gen(GenArgs),
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct GenArgs {
    paths: Vec<Path>,
}

#[derive(Debug)]
enum Path {
    Buf(PathBuf),
    Sec(FileSection),
}

#[derive(Debug)]
struct FileDetails {
    name: String,
    extension: String,
    lines: Vec<Line>,
}

fn main() {
    match Cli::parse() {
        Cli::Gen(args) => generate(args),
    }
}

async fn read_files(
    workers: u8,
    files: Vec<Path>
) {
    
}
