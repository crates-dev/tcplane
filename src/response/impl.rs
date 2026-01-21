use crate::*;

impl StdError for ResponseError {}

impl Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResponseError(data) => write!(f, "Response Error{COLON_SPACE}{data}"),
            Self::CloseError(data) => write!(f, "Close Error{COLON_SPACE}{data}"),
            Self::NotFoundStream => {
                write!(f, "Not found stream")
            }
            Self::Unknown => write!(f, "Unknown"),
        }
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
            .write_all(self.get_response_data())
            .await
            .map_err(|err| ResponseError::ResponseError(err.to_string()))?;
        Ok(())
    }

    pub async fn close(&mut self, stream_lock: &ArcRwLockStream) -> ResponseResult {
        let mut stream: RwLockWriteGuardTcpStream = stream_lock.get_write_lock().await;
        stream
            .shutdown()
            .await
            .map_err(|err| ResponseError::CloseError(err.to_string()))?;
        Ok(())
    }

    pub async fn flush(&mut self, stream_lock: &ArcRwLockStream) -> ResponseResult {
        let mut stream: RwLockWriteGuardTcpStream = stream_lock.get_write_lock().await;
        stream
            .flush()
            .await
            .map_err(|err| ResponseError::ResponseError(err.to_string()))?;
        Ok(())
    }
}
