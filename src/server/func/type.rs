use crate::*;

pub type FuncBox = Box<dyn Func>;
pub type FuncArcLock = ArcRwLock<FuncBox>;
