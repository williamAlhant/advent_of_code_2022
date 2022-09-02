#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Parsing error")]
    Parsing,
    #[error("Not UTF-8")]
    NotUtf8,
}

pub type Result<T> = std::result::Result<T, Error>;