#[derive(Debug)]
pub enum ResponseError {
    ResponseError(String),
    CloseError(String),
    NotFoundStream,
    Unknown,
}
