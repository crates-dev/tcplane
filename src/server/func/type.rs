use crate::*;

pub type FuncBox = Box<dyn Func>;
pub type FuncArcLock = ArcRwLock<FuncBox>;

pub type AsyncFuncBox = Box<dyn AsyncFunc>;
pub type AsyncFuncArcLock = AsyncArcRwLock<AsyncFuncBox>;
