/// Default inner print setting.
pub const DEFAULT_INNER_PRINT: bool = true;

/// Default inner log setting.
pub const DEFAULT_INNER_LOG: bool = true;

/// Colon space separator string.
pub const COLON_SPACE: &str = ": ";

/// Colon space symbol string.
pub const COLON_SPACE_SYMBOL: &str = ":";

/// Default host address.
pub const DEFAULT_HOST: &str = "0.0.0.0";

/// Default listen port number.
pub const DEFAULT_LISTEN_PORT: usize = 60000;

/// Request split marker string.
pub const SPLIT_REQUEST: &str = "\r\n\r\n";

/// Request split marker bytes.
pub const SPLIT_REQUEST_BYTES: &[u8] = SPLIT_REQUEST.as_bytes();

/// Default buffer size for requests.
pub const DEFAULT_BUFFER_SIZE: usize = 512_000;
