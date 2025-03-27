use crate::*;

pub type ArcStream = Arc<TcpStream>;
pub type OptionArcTcpStream = Option<ArcStream>;
pub type OptionArcRwLockStream = Option<ArcRwLockStream>;
pub type RwLockReadGuardTcpStream<'a> = RwLockReadGuard<'a, TcpStream>;
pub type RwLockWriteGuardTcpStream<'a> = RwLockWriteGuard<'a, TcpStream>;
pub type ArcRwLockWriteGuardTcpStream<'a> = Arc<RwLockWriteGuard<'a, TcpStream>>;
pub type OptionArcRwLockWriteGuardTcpStream<'a> = Option<ArcRwLockWriteGuardTcpStream<'a>>;
pub type ArcMutexGuardTcpStream<'a> = Arc<MutexGuard<'a, TcpStream>>;
pub type OptionArcMutexGuardTcpStream<'a> = Option<ArcMutexGuardTcpStream<'a>>;
pub type OptionSocketHost = Option<IpAddr>;
pub type OptionSocketPort = Option<u16>;
pub type OptionSocketAddr = Option<SocketAddr>;

#[derive(Clone, Debug)]
pub struct ArcRwLockStream(pub(super) ArcRwLock<TcpStream>);
