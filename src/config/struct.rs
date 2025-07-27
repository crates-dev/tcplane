use crate::*;

/// Server configuration parameters.
///
/// Contains all necessary settings for server initialization and operation.
#[derive(Clone)]
pub struct ServerConfig {
    /// Server host address.
    pub(crate) host: String,
    /// Server listening port.
    pub(crate) port: usize,
    /// Network buffer size in bytes.
    pub(crate) buffer_size: usize,
    /// Error handler for server operations.
    pub(crate) error_handle: ArcErrorHandle,
}
