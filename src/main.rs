use std::{fs, path::PathBuf, sync::{atomic::{AtomicU32, Ordering}, Arc}};
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

fn main() {
    let workers = 8;
    let pool = ThreadPool::new(workers);
    let Cli::Gen(args) = Cli::parse();

    let (sender, receiver) = crossbeam_channel::unbounded();
    let pending = Arc::new(AtomicU32::new(0));

    for buf in args.paths {
        pending.fetch_add(1, Ordering::Relaxed);
        sender.send(Some(buf)).unwrap();
    }

    for _ in 0..workers {
        let sender = sender.clone();
        let receiver = receiver.clone();
        let pending = pending.clone();

        pool.execute(move || {
            while let Ok(Some(path)) = receiver.recv() {
                if path.is_dir() {
                    if let Ok(dir) = fs::read_dir(&path) {
                        dir.for_each(|dir_entry| match dir_entry {
                            Ok(dir_entry) => {
                                pending.fetch_add(1, Ordering::Relaxed);
                                sender.send(Some(dir_entry.path())).unwrap();
                            },
                            Err(_) => eprintln!("error reading dir"),
                        });
                    }
                } else if path.is_file() {
                    println!("{} - {}", path.display(), fs::read_to_string(&path).unwrap_or_else(|_| "err".into()));
                }

                if pending.fetch_sub(1, Ordering::Relaxed) == 1 {
                    for _ in 0..workers {
                        sender.send(None).unwrap();
                    }
                }
            }
        });
    }

    pool.join();
}
