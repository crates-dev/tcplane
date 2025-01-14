use super::r#trait::{AsyncFunc, Func};
use crate::*;
use http_type::*;
use server::r#type::*;

pub type FuncBox = Box<dyn Func>;
pub type FuncArcLock = ArcRwLock<FuncBox>;

pub type AsyncFuncBox = Box<dyn AsyncFunc>;
pub type AsyncFuncArcLock = AsyncArcRwLock<AsyncFuncBox>;
