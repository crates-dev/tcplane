use super::r#type::ServerConfig;
use crate::utils::thread::get_thread_count;
use crate::*;
use http_type::*;

impl Default for ServerConfig {
    #[inline]
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST.to_owned(),
            port: DEFAULT_WEB_PORT,
            thread_pool_size: get_thread_count(),
            log_dir: DEFAULT_LOG_DIR.to_owned(),
            log_size: 1_024_000_000,
            buffer_size: 1_024_000,
        }
    }
}
