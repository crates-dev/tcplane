use crate::*;

/// Binary data for HTTP response.
pub type ResponseData = Vec<u8>;

/// Result type for response operations.
pub type ResponseResult = Result<(), ResponseError>;
