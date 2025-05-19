use crate::*;

#[derive(Clone, Data)]
pub struct InnerContext {
    pub(super) stream: OptionArcRwLockStream,
    pub(super) request: Request,
    pub(super) response: Response,
    pub(super) log: Log,
    pub(super) data: HashMapArcAnySendSync,
}

#[derive(Clone)]
pub struct Context(pub(super) ArcRwLock<InnerContext>);
