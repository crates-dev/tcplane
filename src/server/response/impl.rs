use super::error::Error;
use crate::*;

impl Default for Response {
    #[inline]
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl Response {
    #[inline]
    pub fn from<T: Into<ResponseData>>(data: T) -> Self {
        Self(data.into())
    }

    #[inline]
    pub fn get_response_data(&self) -> &ResponseData {
        &self.0
    }

    #[inline]
    pub fn set_response_data<T: Into<ResponseData>>(&mut self, data: T) -> &mut Self {
        self.0 = data.into();
        self
    }

    #[inline]
    pub async fn send(&mut self, stream_lock: &ArcRwLockStream) -> ResponseResult {
        let mut stream: RwLockWriteGuardTcpStream = stream_lock.get_write_lock().await;
        let send_res: ResponseResult = stream
            .write_all(&self.get_response_data())
            .await
            .map_err(|err| Error::ResponseError(err.to_string()))
            .and_then(|_| Ok(self.get_response_data().clone()));
        send_res
    }

    #[inline]
    pub async fn close(&mut self, stream_lock: &ArcRwLockStream) {
        let mut stream: RwLockWriteGuardTcpStream = stream_lock.get_write_lock().await;
        stream.close().await;
    }
}
