use crate::*;

pub type ControllerDataStream = Arc<TcpStream>;
pub type ControllerDataStreamOpt = Option<ControllerDataStream>;

#[derive(Clone, Debug, Lombok)]
pub struct ControllerData {
    pub(super) stream: ControllerDataStreamOpt,
    pub(super) request: Request,
    pub(super) response: Response,
    pub(super) log: Log,
}
