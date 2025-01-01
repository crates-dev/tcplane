use lombok_macros::*;

#[derive(Clone, Debug, Lombok)]
pub struct ServerConfig {
    pub(super) host: String,
    pub(super) port: usize,
    pub(super) thread_pool_size: usize,
    pub(super) log_dir: String,
    pub(super) log_size: usize,
    pub(super) buffer_size: usize,
}
