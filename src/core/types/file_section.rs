use crate::core::types::{Section, SectionFromStrError};
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

#[derive(Debug)]
pub struct FileSection {
    file_path: PathBuf,
    section: Section,
}

impl FileSection {
    pub const fn start(&self) -> usize {
        self.section.start
    }

    pub const fn end(&self) -> usize {
        self.section.end
    }

    pub const fn file_path(&self) -> &PathBuf {
        &self.file_path
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
            Ok(Self { file_path: file, section })
        } else {
            Err(FileSectionNewError::IsNotFile(file))
        }
    }
}
