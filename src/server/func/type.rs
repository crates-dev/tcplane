use crate::*;

pub type BoxFunc = Box<dyn Func + Send + 'static>;
