use std::{fs, sync::mpsc, convert::TryFrom, fmt::Display, marker::PhantomData, path::PathBuf};
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

trait PathType {}

struct File {}
impl PathType for File {}

struct Dir {}
impl PathType for Dir {}

#[derive(Debug, Clone)]
struct Path<T: PathType> {
    buf: PathBuf,
    _type: PhantomData<T>
}

impl Path<Dir> {
    fn read(&self) -> fs::ReadDir {
        fs::read_dir(&self.buf).expect("should always be directory")
    }
}

impl Path<File> {
    fn read(&self) -> String {
        fs::read_to_string(&self.buf).expect("should always be readable file")
    }
}

impl<T: PathType> Display for Path<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.buf.display())
    }
}

impl TryFrom<PathBuf> for Path<Dir> {
    type Error = BufToPathError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        if value.is_dir() {
            Ok(Self { buf: value, _type: PhantomData::<_> })
        } else {
            Err(BufToPathError::Dir(value))
        }
    }
}

impl TryFrom<PathBuf> for Path<File> {
    type Error = BufToPathError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        if value.is_file() {
            Ok(Self { buf: value, _type: PhantomData::<_> })
        } else {
            Err(BufToPathError::File(value))
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum BufToPathError {
    #[error("Error when converting buf to directory: {0}")]
    Dir(PathBuf),
    #[error("Error when converting buf to file: {0}")]
    File(PathBuf)
}

enum PathTree<'a> {
    Dir(&'a Path<Dir>, Vec<&'a PathTree<'a>>),
    File(&'a Path<File>)
}

impl<'a> PathTree<'a> {
    fn from_dir(path: &'a Path<Dir>) -> Self {
        Self::Dir(path, Vec::new())
    }

    fn from_file(path: &'a Path<File>) -> Self {
        Self::File(path)
    }
}

fn main() {
    let pool = ThreadPool::new(8);
    let Cli::Gen(args) = Cli::parse();

    let (input, output) = mpsc::channel();
    for buf in args.paths {
        let input = input.clone();
        pool.execute(move || {
            let path: Path<File> = buf.try_into().unwrap();
            input.send(path).unwrap();
        });
    }

    drop(input);

    for path in output {
        println!("{path}");
    }
}
