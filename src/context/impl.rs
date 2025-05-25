use crate::*;

impl InnerContext {
    pub fn new() -> Self {
        InnerContext {
            stream: None,
            request: Request::new(),
            response: Response::default(),
            data: HashMap::default(),
        }
    }
}

impl Context {
    pub(crate) fn from_inner_context(ctx: InnerContext) -> Self {
        Self(Arc::new(RwLock::new(ctx)))
    }

    pub async fn get_read_lock(&self) -> RwLockReadContext {
        self.0.read().await
    }

    pub async fn get_write_lock(&self) -> RwLockWriteContext {
        self.0.write().await
    }

    pub async fn get(&self) -> InnerContext {
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
        Err(ResponseError::NotFoundStream)
    }

    pub async fn close(&self) -> ResponseResult {
        if let Some(stream) = self.get_stream().await {
            self.get_response().await.close(&stream).await?;
            return Ok(());
        }
        Err(ResponseError::NotFoundStream)
    }

    pub async fn flush(&self) -> ResponseResult {
        if let Some(stream) = self.get_stream().await {
            self.get_response().await.flush(&stream).await?;
            return Ok(());
        }
        Err(ResponseError::NotFoundStream)
    }

    pub async fn set_data_value<T: Any + Send + Sync + Clone>(
        &self,
        key: &str,
        value: &T,
    ) -> &Self {
        self.get_write_lock()
            .await
            .get_mut_data()
            .insert(key.to_owned(), Arc::new(value.clone()));
        self
    }

    pub async fn get_data_value<T: Any + Send + Sync + Clone>(&self, key: &str) -> Option<T> {
        self.get_read_lock()
            .await
            .get_data()
            .get(key)
            .and_then(|arc| arc.downcast_ref::<T>())
            .cloned()
    }

    pub async fn remove_data_value(&self, key: &str) -> &Self {
        self.get_write_lock().await.get_mut_data().remove(key);
        self
    }

    pub async fn clear_data(&self) -> &Self {
        self.get_write_lock().await.get_mut_data().clear();
        self
    }
}
