use crate::*;

pub type MiddlewareArcLock = ArcRwLock<Vec<FuncBox>>;
pub type AsyncMiddlewareArcLock = AsyncArcRwLock<Vec<AsyncFuncBox>>;
