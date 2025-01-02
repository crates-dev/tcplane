use crate::server::response::r#type::Response;
use hyperlane_log::*;
use lombok_macros::*;
use std::{net::TcpStream, sync::Arc};

pub type ControllerDataStream = Arc<TcpStream>;
pub type ControllerDataStreamOpt = Option<ControllerDataStream>;
pub type ControllerDataRequest = Vec<u8>;

#[derive(Clone, Debug, Lombok)]
pub struct ControllerData {
    pub(super) stream: ControllerDataStreamOpt,
    pub(super) request: ControllerDataRequest,
    pub(super) response: Response,
    pub(super) log: Log,
}
