use hyperlane_log::*;
use lombok_macros::*;
use std::{net::TcpStream, sync::Arc};

use crate::server::response::r#type::Response;

pub type ControllerDataStream = Arc<TcpStream>;
pub type ControllerDataStreamOpt = Option<ControllerDataStream>;
pub type ControllerDataRequest = Vec<u8>;
pub type ControllerDataRequestOpt = Option<ControllerDataRequest>;

#[derive(Clone, Debug, Lombok)]
pub struct ControllerData {
    pub(super) stream: ControllerDataStreamOpt,
    pub(super) request: ControllerDataRequestOpt,
    pub(super) response: Response,
    pub(super) log: Log,
}
