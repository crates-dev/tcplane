use crate::*;

pub type AsyncArcRwLock<T> = Arc<RwLock<T>>;

#[derive(Clone, Lombok)]
pub struct Server {
    pub(super) cfg: ArcRwLock<ServerConfig>,
    pub(super) func: FuncArcLock,
    pub(super) middleware: MiddlewareArcLock,
    pub(super) tmp: ArcRwLock<Tmp>,
}
