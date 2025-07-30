use crate::*;

/// Error handling trait.
///
/// Defines the interface for error handling functions.
pub trait ErrorHandle: Fn(String) {}

/// Async function trait (without Pin).
///
/// Defines the interface for async functions handling context.
pub trait AsyncFuncWithoutPin<Fut>: Fn(Context) -> Fut + Send + Sync + 'static
where
    Fut: Future<Output = ()> + Send + 'static,
{
}

/// Function trait.
///
/// Defines the interface for functions handling context.
pub trait Func:
    Fn(Context) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync + 'static
{
}
