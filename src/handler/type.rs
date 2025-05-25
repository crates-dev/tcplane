use crate::*;

pub(crate) type BoxFunc = Box<dyn Func + Send + 'static>;
pub(crate) type ArcRwlockVecBoxFunc = ArcRwLock<Vec<BoxFunc>>;
pub(crate) type ArcErrorHandle = Arc<dyn ErrorHandle + Send + Sync + 'static>;
