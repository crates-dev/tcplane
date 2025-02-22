use crate::*;

impl ArcRwLockStream {
    #[inline]
    pub fn from(arc_rw_lock_stream: ArcRwLock<TcpStream>) -> Self {
        Self(arc_rw_lock_stream)
    }

    #[inline]
    pub fn from_stream(stream: TcpStream) -> Self {
        Self(Arc::new(RwLock::new(stream)))
    }

    #[inline]
    pub async fn get_read_lock(&self) -> RwLockReadGuardTcpStream {
        self.0.read().await
    }

    #[inline]
    pub async fn get_write_lock(&self) -> RwLockWriteGuardTcpStream {
        self.0.write().await
    }
}
