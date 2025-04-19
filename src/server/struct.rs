use crate::*;

#[derive(Clone, Lombok)]
pub struct Server {
    pub(super) cfg: ArcRwLockServerConfig,
    pub(super) func_list: FuncListArcLock,
    pub(super) tmp: ArcRwLock<Tmp>,
}
