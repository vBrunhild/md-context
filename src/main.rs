use std::{fs, path::PathBuf};
use clap::Parser;

#[derive(Parser)]
#[command(name = "mdcon")]
enum Cli {
    Add(AddArgs)
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct AddArgs {
    paths: Vec<PathBuf>
}


#[derive(Debug, Clone)]
enum Path {
    Dir { buf: PathBuf, content: Vec<Path> },
    File { buf: PathBuf, content: Option<String> }
}

fn buf_to_path(buf: PathBuf) -> Option<Path> {
    if buf.is_file() {
        let content = fs::read_to_string(&buf).ok();
        Some(Path::File { buf, content })
    } else if buf.is_dir() {
        let content = fs::read_dir(&buf)
            .expect("Should always be a dir")
            .flat_map(|inner| match inner {
                Ok(dir) => buf_to_path(dir.path()),
                Err(_) => None
            })
            .collect();

        Some(Path::Dir { buf, content })
    } else {
        None
    }
}

fn main() {
    let Cli::Add(args) = Cli::parse();
    let paths: Vec<Path> = args.paths.into_iter()
        .flat_map(|buf| buf_to_path(buf))
        .collect();

    dbg!(paths);
}

