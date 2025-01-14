use crate::ControllerData;
use std_macro_extensions::*;

pub trait AsyncFunc:
    Fn(&mut ControllerData) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>
    + Send
    + Sync
    + 'static
{
}

pub trait Func: Fn(&mut ControllerData) + Send + Sync + 'static {}
