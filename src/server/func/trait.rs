use crate::*;

pub trait AsyncFuncWithoutPin<Fut>:
    Fn(ArcRwLockControllerData) -> Fut + Send + Sync + 'static
where
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
}

pub trait Func: Fn(ArcRwLockControllerData) + Send + Sync + 'static {}
