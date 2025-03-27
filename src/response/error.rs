use crate::*;

#[derive(Debug)]
pub enum Error {
    ResponseError(String),
    CloseError(String),
    NotFoundStream,
    Unknown,
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResponseError(data) => write!(f, "Response Error{}{}", COLON_SPACE, data),
            Self::CloseError(data) => write!(f, "Close Error{}{}", COLON_SPACE, data),
            Self::NotFoundStream => {
                write!(f, "Not found stream")
            }
            Self::Unknown => write!(f, "{}", "Unknown"),
        }
    }
}
