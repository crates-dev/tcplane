use crate::*;

/// A write guard for the `InnerContext`.
pub type RwLockWriteContext<'a> = RwLockWriteGuard<'a, InnerContext>;

/// A read guard for the `InnerContext`.
pub type RwLockReadContext<'a> = RwLockReadGuard<'a, InnerContext>;

/// An `Arc` pointer to a type that is `Any`, `Send`, and `Sync`.
pub type ArcAnySendSync = Arc<dyn Any + Send + Sync>;

/// A hash map storing `ArcAnySendSync` values, keyed by `String`.
pub type HashMapArcAnySendSync = HashMap<String, ArcAnySendSync>;
