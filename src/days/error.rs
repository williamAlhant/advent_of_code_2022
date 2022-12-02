#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Parsing error")]
    Parsing {
        content: Option<String>,
        line: usize
    },
    #[error("Parsing error")]
    ParsingToken,
    #[error("Not UTF-8")]
    NotUtf8,
    #[error("No solution")]
    NoSolution
}

impl Error {
    pub fn new_parsing(content: &str, line: usize) -> Self {
        Error::Parsing {
            content: Some(String::from(content)),
            line
        }
    }

    pub fn new_parsing_no_content(line: usize) -> Self {
        Error::Parsing {
            content: None,
            line
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;