#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Parsing error")]
    Parsing {
        content: String
    },
    #[error("Not UTF-8")]
    NotUtf8,
    #[error("No solution")]
    NoSolution
}

pub type Result<T> = std::result::Result<T, Error>;