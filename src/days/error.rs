#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Parsing error")]
    Parsing(Parsing),
    #[error("Parsing error")]
    ParsingToken(Token),
    #[error("Unexpected lack of input")]
    UnexpectedInputEnd,
    #[error("Not UTF-8")]
    NotUtf8,
    #[error("No solution")]
    NoSolution
}

#[derive(Debug)]
pub struct Parsing {
    pub content: Option<String>,
    pub line: usize,
    pub token: Option<Token>
}

#[derive(Debug)]
pub struct Token {
    pub line_pos: usize,
    pub token_len: usize
}

impl Error {
    pub fn new_parsing(content: &str, line: usize) -> Self {
        Error::Parsing(
            Parsing {
                content: Some(String::from(content)),
                line,
                token: None
            }
        )
    }

    pub fn new_parsing_no_content(line: usize) -> Self {
        Error::Parsing(
            Parsing {
                content: None,
                line,
                token: None
            }
        )
    }

    pub fn new_token(line_pos: usize, token_len: usize) -> Self {
        Error::ParsingToken(
            Token {
                line_pos,
                token_len
            }
        )
    }

    pub fn new_parsing_with_token(content: &str, line: usize, token: Token) -> Self {
        Error::Parsing(
            Parsing {
                content: Some(String::from(content)),
                line,
                token: Some(token)
            }
        )
    }
}

pub type Result<T> = std::result::Result<T, Error>;