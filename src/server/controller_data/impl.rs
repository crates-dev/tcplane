use crate::*;

impl InnerControllerData {
    pub fn new() -> Self {
        InnerControllerData {
            stream: None,
            request: Request::new(),
            response: Response::default(),
            log: Log::default(),
        }
    }
}

impl ControllerData {
    pub(crate) fn from_controller_data(controller_data: InnerControllerData) -> Self {
        Self(Arc::new(RwLock::new(controller_data)))
    }

    pub async fn get_read_lock(&self) -> RwLockReadControllerData {
        self.0.read().await
    }

    pub async fn get_write_lock(&self) -> RwLockWriteControllerData {
        self.0.write().await
    }

    pub async fn get(&self) -> InnerControllerData {
        self.get_read_lock().await.clone()
    }

    pub async fn get_stream(&self) -> OptionArcRwLockStream {
        self.get().await.get_stream().clone()
    }

    pub async fn get_request(&self) -> Request {
        self.get().await.get_request().clone()
    }

    pub async fn get_response(&self) -> Response {
        self.get().await.get_response().clone()
    }

    pub async fn get_log(&self) -> Log {
        self.get().await.get_log().clone()
    }

    pub(super) async fn set_response_data<T: Into<ResponseData>>(&self, data: T) -> &Self {
        self.get_write_lock()
            .await
            .get_mut_response()
            .set_response_data(data);
        self
    }

    pub async fn get_socket_addr(&self) -> OptionSocketAddr {
        let stream_result: OptionArcRwLockStream = self.get_stream().await;
        if stream_result.is_none() {
            return None;
        }
        let socket_addr_opt: OptionSocketAddr = stream_result
            .unwrap()
            .get_read_lock()
            .await
            .peer_addr()
            .ok();
        socket_addr_opt
    }

    pub async fn get_socket_addr_or_default(&self) -> SocketAddr {
        let stream_result: OptionArcRwLockStream = self.get_stream().await;
        if stream_result.is_none() {
            return DEFAULT_SOCKET_ADDR;
        }
        let socket_addr: SocketAddr = stream_result
            .unwrap()
            .get_read_lock()
            .await
            .peer_addr()
            .unwrap_or(DEFAULT_SOCKET_ADDR);
        socket_addr
    }

    pub async fn get_socket_addr_string(&self) -> Option<String> {
        self.get_socket_addr().await.map(|data| data.to_string())
    }

    pub async fn get_socket_addr_or_default_string(&self) -> String {
        self.get_socket_addr_or_default().await.to_string()
    }

    pub async fn get_socket_host(&self) -> OptionSocketHost {
        self.get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.ip())
    }

    pub async fn get_socket_port(&self) -> OptionSocketPort {
        self.get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.port())
    }

    pub async fn log_info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.get_read_lock().await.get_log().info(data, func);
        self
    }

    pub async fn log_debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.get_read_lock().await.get_log().debug(data, func);
        self
    }

    pub async fn log_error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.get_read_lock().await.get_log().error(data, func);
        self
    }

    pub async fn async_log_info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.get_read_lock().await.get_log().async_info(data, func);
        self
    }

    pub async fn async_log_debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.get_read_lock().await.get_log().async_debug(data, func);
        self
    }

    pub async fn async_log_error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        self.get_read_lock().await.get_log().async_error(data, func);
        self
    }

    pub async fn send<T: Into<ResponseData>>(&self, data: T) -> ResponseResult {
        if let Some(stream) = self.get_stream().await {
            self.set_response_data(data)
                .await
                .get_response()
                .await
                .send(&stream)
                .await?;
            return Ok(());
        }
        Err(server::response::error::Error::NotFoundStream)
    }

    pub async fn close(&self) -> ResponseResult {
        if let Some(stream) = self.get_stream().await {
            self.get_response().await.close(&stream).await?;
            return Ok(());
        }
        Err(server::response::error::Error::NotFoundStream)
    }

    pub async fn flush(&self) -> ResponseResult {
        if let Some(stream) = self.get_stream().await {
            self.get_response().await.flush(&stream).await?;
            return Ok(());
        }
        Err(server::response::error::Error::NotFoundStream)
    }
}
