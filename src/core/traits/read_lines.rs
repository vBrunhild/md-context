use crate::core::types::{Line, Section};
use std::io;
use std::io::BufRead;

pub trait ReadLines {
    fn read_lines(self, section: Section) -> impl Iterator<Item = Result<Line, io::Error>>;
}

impl<T: io::Read> ReadLines for T {
    fn read_lines(self, section: Section) -> impl Iterator<Item = Result<Line, io::Error>> {
        io::BufReader::new(self)
            .lines()
            .enumerate()
            .skip(section.start() - 1)
            .map_while(move |(number, content)| {
                let number = number + 1;
                if &number <= section.end() {
                    Some(content.map(|content| Line { number, content }))
                } else {
                    None
                }
            })
    }
}
