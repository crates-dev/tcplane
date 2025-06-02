use crate::*;

#[derive(Clone)]
pub struct InnerContext {
    pub(crate) stream: OptionArcRwLockStream,
    pub(crate) request: Request,
    pub(crate) response: Response,
    pub(crate) data: HashMapArcAnySendSync,
}

#[derive(Clone)]
pub struct Context(pub(super) ArcRwLock<InnerContext>);
