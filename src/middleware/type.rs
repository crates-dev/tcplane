use crate::*;

pub type FuncListArcLock = AsyncArcRwLock<Vec<BoxFunc>>;
