use crate::*;

/// Server structure.
///
/// Contains the core configuration and function list of the server.
#[derive(Clone)]
pub struct Server {
    /// Server configuration containing all necessary settings.
    pub(crate) config: ArcRwLockServerConfig,
    /// Function list containing all supported processing functions.
    pub(crate) func_list: FuncListArcLock,
}
