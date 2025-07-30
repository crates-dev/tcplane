use crate::*;

/// Internal context structure.
///
/// Stores core data during request processing, including stream, request, response, and data.
#[derive(Clone)]
pub struct InnerContext {
    /// The stream object for network communication.
    pub(crate) stream: OptionArcRwLockStream,
    /// The request object containing client-sent request information.
    pub(crate) request: Request,
    /// The response object for building and sending responses to clients.
    pub(crate) response: Response,
    /// Data storage for holding arbitrary type data during request processing.
    pub(crate) data: HashMapArcAnySendSync,
}

/// Context structure.
///
/// Wraps `InnerContext` providing thread-safe shared access.
#[derive(Clone)]
pub struct Context(
    /// The inner context, wrapped in an ArcRwLock for thread-safe access.
    pub(super) ArcRwLock<InnerContext>,
);
