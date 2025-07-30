use crate::*;

/// Implementation of the `ErrorHandle` trait.
///
/// Provides error handling functionality for function types.
impl<T> ErrorHandle for T where T: Fn(String) {}

/// Implementation of the `Func` trait.
///
/// Provides context handling functionality for function types.
impl<F> Func for F where
    F: Fn(Context) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync + 'static
{
}

/// Implementation of the `AsyncFuncWithoutPin` trait.
///
/// Provides context handling functionality for async function types.
impl<F, Fut> AsyncFuncWithoutPin<Fut> for F
where
    F: Fn(Context) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
}
