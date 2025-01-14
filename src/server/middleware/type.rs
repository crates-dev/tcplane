use crate::*;
use http_type::*;

pub type MiddlewareArcLock = ArcRwLock<Vec<FuncBox>>;
pub type AsyncMiddlewareArcLock = AsyncArcRwLock<Vec<AsyncFuncBox>>;
