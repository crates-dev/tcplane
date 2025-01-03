use super::error::Error;
use lombok_macros::*;

pub type ControllerDataResponse = Vec<u8>;
pub type ResponseResult = Result<ControllerDataResponse, Error>;

#[derive(Clone, Debug, Lombok)]
pub struct Response {
    pub(crate) data: ControllerDataResponse,
}
