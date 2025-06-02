use crate::*;

#[derive(Clone)]
pub struct Server {
    pub(crate) config: ArcRwLockServerConfig,
    pub(crate) func_list: FuncListArcLock,
}
