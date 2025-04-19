use crate::*;

#[derive(Clone, Debug)]
pub struct ArcRwLockStream(pub(super) ArcRwLock<TcpStream>);
