use crate::*;
use http_type::*;

pub type AsyncArcRwLock<T> = Arc<tokio::sync::RwLock<T>>;

#[derive(Clone, Lombok)]
pub struct Server {
    pub(crate) cfg: ArcRwLock<ServerConfig>,
    pub(crate) func: ArcRwLock<FuncBox>,
    pub(crate) middleware: ArcRwLock<Vec<FuncBox>>,
    pub(crate) async_func: AsyncArcRwLock<AsyncFuncBox>,
    pub(crate) async_middleware: AsyncArcRwLock<Vec<AsyncFuncBox>>,
    pub(crate) tmp: ArcRwLock<Tmp>,
}
