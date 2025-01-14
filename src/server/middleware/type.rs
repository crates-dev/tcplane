use crate::*;
use http_type::*;
use server::{func::r#type::*, r#type::*};

pub type MiddlewareArcLock = ArcRwLock<Vec<FuncBox>>;
pub type AsyncMiddlewareArcLock = AsyncArcRwLock<Vec<AsyncFuncBox>>;
