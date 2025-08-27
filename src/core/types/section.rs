use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum SectionNewError {
    #[error("starting or ending line can't be zero")]
    Zero,
    #[error("starting line can't be bigger then ending line")]
    StartAfterEnd,
}

#[derive(Debug, thiserror::Error)]
pub enum SectionFromStrError {
    #[error("invalid section: {0}")]
    New(#[from] SectionNewError),
    #[error("error parsing start: {0}")]
    ParseStart(String),
    #[error("error parsing end: {0}")]
    ParseEnd(String),
    #[error("error parsing single line: {0}")]
    SingleLine(String),
}

#[derive(Debug, Clone)]
pub struct Section {
    start: usize,
    end: usize,
}

impl Section {
    pub const fn start(&self) -> &usize {
        &self.start
    }

    pub const fn end(&self) -> &usize {
        &self.end
    }
}

impl TryFrom<usize> for Section {
    type Error = SectionNewError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(SectionNewError::Zero)
        } else {
            Ok(Self {
                start: value,
                end: value,
            })
        }
    }
}

impl TryFrom<(usize, usize)> for Section {
    type Error = SectionNewError;

    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        match value {
            (start, end) if start == 0 || end == 0 => Err(SectionNewError::Zero),
            (start, end) if start > end => Err(SectionNewError::StartAfterEnd),
            (start, end) => Ok(Self { start, end }),
        }
    }
}

impl Default for Section {
    fn default() -> Self {
        Self {
            start: 1,
            end: usize::MAX,
        }
    }
}

impl FromStr for Section {
    type Err = SectionFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((start, end)) = s.split_once(':') {
            let start = start
                .is_empty()
                .then_some(1)
                .or_else(|| start.parse().ok())
                .ok_or_else(|| SectionFromStrError::ParseStart(start.to_string()))?;

            let end = end
                .is_empty()
                .then_some(usize::MAX)
                .or_else(|| end.parse().ok())
                .ok_or_else(|| SectionFromStrError::ParseEnd(end.to_string()))?;

            Ok((start, end).try_into()?)
        } else {
            let line: usize = s
                .parse()
                .map_err(|_| SectionFromStrError::SingleLine(s.to_string()))?;
            Ok(line.try_into()?)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let section: Section = "3:20".parse().unwrap();
        dbg!(section);
    }
}
