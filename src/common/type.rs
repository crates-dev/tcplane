use crate::*;

/// A type alias for `Arc<RwLock<T>>`.
///
/// This type alias simplifies the use of `Arc<RwLock<T>>`, providing thread-safe shared ownership with read-write access.
pub type ArcRwLock<T> = Arc<RwLock<T>>;
