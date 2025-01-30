use zayden_core::ErrorResponse;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NotInteractionAuthor,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl ErrorResponse for Error {
    fn to_response(&self) -> &str {
        match self {
            Error::NotInteractionAuthor => zayden_core::Error::NotInteractionAuthor.to_response(),
        }
    }
}
