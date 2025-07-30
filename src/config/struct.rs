use crate::*;

/// Server configuration parameters.
///
/// Contains all settings required for server initialization and operation.
#[derive(Clone)]
pub struct ServerConfig {
    /// The server host address.
    pub(crate) host: String,
    /// The server listening port.
    pub(crate) port: usize,
    /// Network buffer size in bytes.
    pub(crate) buffer_size: usize,
    /// Error handling function for server operations.
    pub(crate) error_handle: ArcErrorHandle,
}
