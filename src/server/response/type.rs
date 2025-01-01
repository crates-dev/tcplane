use super::error::Error;
use lombok_macros::*;

pub type ControllerDataResponse = Vec<u8>;
pub type ControllerDataResponseOpt = Option<ControllerDataResponse>;
pub type ResponseResult = Result<ControllerDataResponseOpt, Error>;

#[derive(Clone, Debug, Lombok)]
pub struct Response {
    pub(crate) data: ControllerDataResponseOpt,
}
