use crate::*;

pub type RwLockWriteContext<'a> = RwLockWriteGuard<'a, InnerContext>;
pub type RwLockReadContext<'a> = RwLockReadGuard<'a, InnerContext>;

#[derive(Clone, Lombok)]
pub struct InnerContext {
    pub(super) stream: OptionArcRwLockStream,
    pub(super) request: Request,
    pub(super) response: Response,
    pub(super) log: Log,
}

#[derive(Clone)]
pub struct Context(pub(super) ArcRwLock<InnerContext>);
