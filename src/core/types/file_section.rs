use crate::core::{
    traits::ReadLines,
    types::{Line, Section, SectionFromStrError},
};
use std::{fs::File, io, path::PathBuf, str::FromStr};

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
    pub fn read_lines(self) -> Result<impl Iterator<Item = Result<Line, io::Error>>, io::Error> {
        Ok(File::open(self.file)?.read_lines(self.section))
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
