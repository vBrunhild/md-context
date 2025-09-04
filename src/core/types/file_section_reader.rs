use crate::core::{traits::AsyncTryFrom, types::FileSection};
use smol::io::AsyncBufReadExt;
use smol::stream::{Stream, StreamExt};
use smol::{
    fs::File,
    io::{self, BufReader},
};
use std::task::Poll;

pub struct FileSectionReader {
    file_section: FileSection,
    reader: BufReader<File>,
    current_offset: usize,
}

impl AsyncTryFrom<FileSection> for FileSectionReader {
    type Error = io::Error;

    async fn try_from(value: FileSection) -> Result<Self, Self::Error> {
        Ok(Self {
            reader: BufReader::new(File::open(value.file_path()).await?),
            current_offset: value.start(),
            file_section: value,
        })
    }
}

impl Stream for FileSectionReader {
    type Item = (usize, io::Result<String>);

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let mut lines = Box::pin(self.reader)
            .lines()
            .enumerate()
            .skip(self.file_section.start() - 1);

        loop {
            if self.current_offset >= self.file_section.end() {
                return Poll::Ready(None);
            }

            return match lines.poll_next(cx) {
                Poll::Ready(line) => {
                    self.current_offset += 1;
                    Poll::Ready(line)
                }
                _ => Poll::Pending,
            };
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.file_section.start(), Some(self.file_section.end()))
    }
}
