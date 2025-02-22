use super::error::Error;
use crate::*;

impl Default for Response {
    #[inline]
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

impl Response {
    #[inline]
    pub async fn send(&mut self, stream_lock: &ArcRwLockStream) -> ResponseResult {
        let mut stream: RwLockWriteGuard<'_, TcpStream> = stream_lock.get_write_lock().await;
        let send_res: Result<Vec<u8>, Error> = stream
            .write_all(&self.get_data())
            .await
            .map_err(|err| Error::ResponseError(err.to_string()))
            .and_then(|_| Ok(self.get_data()))
            .cloned();
        let _ = stream.flush().await;
        send_res
    }

    #[inline]
    pub fn set_data<T: Into<ResponseData>>(&mut self, data: T) -> &mut Self {
        self.data = data.into();
        self
    }
}
