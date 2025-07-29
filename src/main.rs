use std::{fs, path::PathBuf, sync::{mpsc, Arc, Mutex}};
use clap::Parser;
use threadpool::ThreadPool;

#[derive(Debug, Parser)]
#[command(name = "mdcon")]
enum Cli {
    Gen(GenArgs)
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct GenArgs {
    paths: Vec<PathBuf>
}

enum PathTree<'a> {
    Dir { buf: &'a PathBuf, entries: Vec<&'a PathTree<'a>> },
    File { buf: &'a PathBuf }
}

fn main() {
    let workers = 8;
    let pool = ThreadPool::new(workers);
    let Cli::Gen(args) = Cli::parse();

    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    for buf in args.paths {
        sender.send(buf).unwrap();
    }

    for _ in 0..workers {
        let sender = sender.clone();
        let receiver = receiver.clone();

        pool.execute(move || {
            // TODO! Very wonky implementation... should be reworked - study crossbeam::channel?
            loop {
                let buf = {
                    let guard = receiver.lock().unwrap();
                    guard.recv()
                };

                match buf {
                    Ok(buf) => {
                        if buf.is_dir() {
                            for dir_entry in fs::read_dir(&buf).unwrap() {
                                match dir_entry {
                                    Ok(entry) => sender.send(entry.path()).unwrap(),
                                    Err(_) => eprintln!("Error readir dir_entry")
                                }
                            }
                        }

                        if buf.is_file() {
                            println!("{}\n{}", buf.display(), fs::read_to_string(&buf).unwrap());
                        }
                    }
                    Err(_) => break,
                }
            }
        });
    }

    drop(sender);
    pool.join();
}
