use crate::*;

pub type ArcTcpStream = Arc<TcpStream>;
pub type OptionArcTcpStream = Option<ArcTcpStream>;

#[derive(Clone, Debug, Lombok)]
pub struct ControllerData {
    pub(super) stream: OptionArcTcpStream,
    pub(super) request: Request,
    pub(super) response: Response,
    pub(super) log: Log,
}
