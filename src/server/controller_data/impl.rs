use crate::*;

impl ControllerData {
    #[inline]
    pub fn new() -> Self {
        ControllerData {
            stream: None,
            request: Request::new(),
            response: Response::default(),
            log: Log::default(),
        }
    }
}

impl ArcRwLockControllerData {
    #[inline]
    pub(crate) fn from_controller_data(controller_data: ControllerData) -> Self {
        Self(Arc::new(RwLock::new(controller_data)))
    }

    #[inline]
    pub async fn get_read_lock(&self) -> RwLockReadControllerData {
        let controller_data: RwLockReadControllerData = self.0.read().await;
        controller_data
    }

    #[inline]
    pub async fn get_write_lock(&self) -> RwLockWriteControllerData {
        let controller_data: RwLockWriteControllerData = self.0.write().await;
        controller_data
    }

    #[inline]
    pub async fn get_controller_data(&self) -> ControllerData {
        let controller_data: ControllerData = self.get_read_lock().await.clone();
        controller_data
    }

    #[inline]
    pub async fn get_stream(&self) -> OptionArcRwLockStream {
        let controller_data: ControllerData = self.get_controller_data().await;
        controller_data.get_stream().clone()
    }

    #[inline]
    pub async fn get_request(&self) -> Request {
        let controller_data: ControllerData = self.get_controller_data().await;
        controller_data.get_request().clone()
    }

    #[inline]
    pub async fn get_response(&self) -> Response {
        let controller_data: ControllerData = self.get_controller_data().await;
        controller_data.get_response().clone()
    }

    #[inline]
    pub async fn get_log(&self) -> Log {
        let controller_data: ControllerData = self.get_controller_data().await;
        controller_data.get_log().clone()
    }

    #[inline]
    pub(super) async fn set_response_data<T: Into<ResponseData>>(&self, data: T) -> &Self {
        let mut controller_data: RwLockWriteControllerData = self.get_write_lock().await;
        let response: &mut Response = controller_data.get_mut_response();
        response.set_response_data(data);
        self
    }

    #[inline]
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

    #[inline]
    pub async fn get_socket_host(&self) -> OptionSocketHost {
        let addr: Option<SocketAddr> = self.get_socket_addr().await;
        let socket_host_opt: OptionSocketHost =
            addr.map(|socket_addr: SocketAddr| socket_addr.ip());
        socket_host_opt
    }

    #[inline]
    pub async fn get_socket_port(&self) -> OptionSocketPort {
        let addr: Option<SocketAddr> = self.get_socket_addr().await;
        let socket_port_opt: OptionSocketPort =
            addr.map(|socket_addr: SocketAddr| socket_addr.port());
        socket_port_opt
    }

    #[inline]
    pub async fn log_info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let log: &Log = controller_data.get_log();
        log.info(data, func);
        self
    }

    #[inline]
    pub async fn log_debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let log: &Log = controller_data.get_log();
        log.debug(data, func);
        self
    }

    #[inline]
    pub async fn log_error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        let controller_data: RwLockReadControllerData = self.get_read_lock().await;
        let log: &Log = controller_data.get_log();
        log.error(data, func);
        self
    }

    #[inline]
    pub async fn send<T: Into<ResponseData>>(&self, data: T) -> ResponseResult {
        if let Some(stream) = self.get_stream().await {
            let response_data: ResponseData = self
                .set_response_data(data)
                .await
                .get_response()
                .await
                .send(&stream)
                .await?;
            return Ok(response_data);
        }
        Err(server::response::error::Error::Unknown)
    }
}
