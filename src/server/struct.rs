use crate::*;

#[derive(Clone, Data)]
pub struct Server {
    pub(super) cfg: ArcRwLockServerConfig,
    pub(super) func_list: FuncListArcLock,
    pub(super) tmp: ArcRwLock<Tmp>,
}
