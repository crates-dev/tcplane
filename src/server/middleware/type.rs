use crate::*;

pub type MiddlewareArcLock = AsyncArcRwLock<Vec<FuncBox>>;
