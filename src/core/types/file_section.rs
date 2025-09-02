use crate::core::types::{Line, Section, SectionFromStrError};
use smol::{
    fs::File,
    io::{self, AsyncBufReadExt},
    stream::StreamExt,
};
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, thiserror::Error)]
pub enum FileSectionNewError {
    #[error("path is not file: {0}")]
    IsNotFile(PathBuf),
    #[error("{0}")]
    Section(#[from] SectionFromStrError),
    #[error("failed to split file and section")]
    SplitFail(String),
}

#[derive(Debug, Clone)]
pub struct FileSection {
    file: PathBuf,
    section: Section,
}

impl FileSection {
    pub async fn read_lines(self) -> Vec<(usize, Result<String, io::Error>)> {
        io::BufReader::new(File::open(self.file).await.unwrap())
            .lines()
            .enumerate()
            .skip(self.section.start - 1)
            .map_while(|(index, content)| {
                index += 1;
                if index < self.section.size() - 1 {
                    Some((index, content))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl FromStr for FileSection {
    type Err = FileSectionNewError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (file, section) = s
            .split_once('#')
            .ok_or_else(|| FileSectionNewError::SplitFail(s.to_string()))?;

        let file: PathBuf = file.parse().expect("is infallible");
        let section: Section = section.parse()?;

        if file.is_file() {
            Ok(Self { file, section })
        } else {
            Err(FileSectionNewError::IsNotFile(file))
        }
    }
}
