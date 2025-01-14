use super::r#trait::*;
use crate::ControllerData;
use std_macro_extensions::*;

impl<F> Func for F where F: Fn(&mut ControllerData) + Send + Sync + 'static {}

impl<F> AsyncFunc for F where
    F: Fn(&mut ControllerData) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>
        + Send
        + Sync
        + 'static
{
}

impl<F, Fut> AsyncFuncWithoutPin<Fut> for F
where
    F: Fn(&mut ControllerData) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
}
