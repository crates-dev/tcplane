use super::error::Error;
use crate::*;

impl Default for Response {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl Response {
    pub fn from<T: Into<ResponseData>>(data: T) -> Self {
        Self(data.into())
    }

    pub fn get_response_data(&self) -> &ResponseData {
        &self.0
    }

    pub fn set_response_data<T: Into<ResponseData>>(&mut self, data: T) -> &mut Self {
        self.0 = data.into();
        self
    }

    pub async fn send(&mut self, stream_lock: &ArcRwLockStream) -> ResponseResult {
        let mut stream: RwLockWriteGuardTcpStream = stream_lock.get_write_lock().await;
        stream
            .write_all(&self.get_response_data())
            .await
            .map_err(|err| Error::ResponseError(err.to_string()))?;
        Ok(())
    }

    pub async fn close(&mut self, stream_lock: &ArcRwLockStream) -> ResponseResult {
        let mut stream: RwLockWriteGuardTcpStream = stream_lock.get_write_lock().await;
        stream
            .shutdown()
            .await
            .map_err(|err| Error::CloseError(err.to_string()))?;
        Ok(())
    }

    pub async fn flush(&mut self, stream_lock: &ArcRwLockStream) -> ResponseResult {
        let mut stream: RwLockWriteGuardTcpStream = stream_lock.get_write_lock().await;
        stream
            .flush()
            .await
            .map_err(|err| Error::ResponseError(err.to_string()))?;
        Ok(())
    }
}
