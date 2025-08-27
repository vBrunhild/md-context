mod core;

use crate::core::types::FileSection;
use clap::Parser;
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU32, Ordering},
    thread,
};

#[derive(Debug, Parser)]
#[command(name = "mdcon")]
enum Cli {
    Gen(GenArgs),
    Test(TestArgs),
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct GenArgs {
    paths: Vec<PathBuf>,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct TestArgs {
    sections: Vec<FileSection>,
}

fn main() {
    match Cli::parse() {
        Cli::Gen(args) => generate(args),
        Cli::Test(args) => {
            for file_section in args.sections {
                for line in file_section.read_lines().unwrap() {
                    dbg!(line.unwrap());
                }
            }
        }
    }
}

fn generate(args: GenArgs) {
    let workers = 8;

    let (sender, receiver) = crossbeam_channel::unbounded();
    let pending = AtomicU32::new(0);

    for path in args.paths {
        pending.fetch_add(1, Ordering::Relaxed);
        sender.send(Some(path)).expect("should send arg paths");
    }

    thread::scope(|s| {
        for _ in 0..workers {
            s.spawn(|| {
                let sender = sender.clone();
                let receiver = receiver.clone();

                while let Ok(Some(path)) = receiver.recv() {
                    if path.is_dir() {
                        if let Ok(dir) = fs::read_dir(&path) {
                            dir.for_each(|dir_entry| match dir_entry {
                                Ok(dir_entry) => {
                                    pending.fetch_add(1, Ordering::Relaxed);
                                    sender
                                        .send(Some(dir_entry.path()))
                                        .expect("should send path");
                                }
                                Err(_) => eprintln!("error reading dir"),
                            });
                        }
                    } else if path.is_file() {
                        println!("found file {}", path.display());
                    }

                    if pending.fetch_sub(1, Ordering::Relaxed) == 1 {
                        for _ in 0..workers {
                            sender.send(None).expect("should send poison pill");
                        }
                    }
                }
            });
        }
    });
}
