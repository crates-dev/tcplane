use super::error::Error;
use crate::*;

pub type ResponseData = Vec<u8>;
pub type ResponseResult = Result<ResponseData, Error>;

#[derive(Clone, Debug, Lombok)]
pub struct Response {
    #[set(skip)]
    pub(super) data: ResponseData,
}
