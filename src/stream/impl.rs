use crate::*;

impl ArcRwLockStream {
    pub fn from(arc_rw_lock_stream: ArcRwLock<TcpStream>) -> Self {
        Self(arc_rw_lock_stream)
    }

    pub fn from_stream(stream: TcpStream) -> Self {
        Self(Arc::new(RwLock::new(stream)))
    }

    pub async fn get_read_lock(&'_ self) -> RwLockReadGuardTcpStream<'_> {
        self.0.read().await
    }

    pub async fn get_write_lock(&'_ self) -> RwLockWriteGuardTcpStream<'_> {
        self.0.write().await
    }
}
