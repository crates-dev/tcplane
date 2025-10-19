use crate::*;

/// Manages the internal state of the context.
impl Default for InnerContext {
    fn default() -> Self {
        Self {
            stream: None,
            request: Request::new(),
            response: Response::default(),
            data: HashMap::default(),
        }
    }
}

impl InnerContext {
    /// Creates a new `InnerContext` with default values.
    ///
    /// # Returns
    ///
    /// - `InnerContext` - A new instance of `InnerContext`.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Provides thread-safe access and manipulation of the context.
impl Context {
    /// Creates a `Context` from an `InnerContext`.
    ///
    /// # Arguments
    ///
    /// - `InnerContext` - The inner context to wrap.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `Context` instance.
    pub(crate) fn from_inner_context(ctx: InnerContext) -> Self {
        Self(Arc::new(RwLock::new(ctx)))
    }

    /// Gets a read lock for the inner context.
    ///
    /// # Arguments
    ///
    /// - `&self` - A reference to the `Context`.
    ///
    /// # Returns
    ///
    /// - `RwLockReadContext` - A read guard for the inner context.
    pub async fn get_read_lock(&'_ self) -> RwLockReadContext<'_> {
        self.0.read().await
    }

    /// Gets a write lock for the inner context.
    ///
    /// # Arguments
    ///
    /// - `&self` - A reference to the `Context`.
    ///
    /// # Returns
    ///
    /// - `RwLockWriteContext` - A write guard for the inner context.
    pub async fn get_write_lock(&'_ self) -> RwLockWriteContext<'_> {
        self.0.write().await
    }

    /// Gets a clone of the inner context.
    ///
    /// # Arguments
    ///
    /// - `&self` - A reference to the `Context`.
    ///
    /// # Returns
    ///
    /// - `InnerContext` - A cloned inner context.
    pub async fn get(&self) -> InnerContext {
        self.get_read_lock().await.clone()
    }

    /// Gets the stream from the inner context.
    ///
    /// # Arguments
    ///
    /// - `&self` - A reference to the `Context`.
    ///
    /// # Returns
    ///
    /// - `OptionArcRwLockStream` - The optional stream.
    pub async fn get_stream(&self) -> OptionArcRwLockStream {
        self.get().await.stream.clone()
    }

    /// Gets the request from the inner context.
    ///
    /// # Arguments
    ///
    /// - `&self` - A reference to the `Context`.
    ///
    /// # Returns
    ///
    /// - `Request` - The request object.
    pub async fn get_request(&self) -> Request {
        self.get().await.request.clone()
    }

    /// Gets the response from the inner context.
    ///
    /// # Arguments
    ///
    /// - `&self` - A reference to the `Context`.
    ///
    /// # Returns
    ///
    /// - `Response` - The response object.
    pub async fn get_response(&self) -> Response {
        self.get().await.response.clone()
    }

    /// Sets response data.
    ///
    /// # Arguments
    ///
    /// - `&self` - A reference to the `Context`.
    /// - `data` - The data to set, which can be converted into `ResponseData`.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the `Context`.
    pub(super) async fn set_response_data<T: Into<ResponseData>>(&self, data: T) -> &Self {
        self.get_write_lock().await.response.set_response_data(data);
        self
    }

    /// Gets the socket address from the stream.
    ///
    /// # Arguments
    ///
    /// - `&self` - A reference to the `Context`.
    ///
    /// # Returns
    ///
    /// - `OptionSocketAddr` - The optional socket address.
    pub async fn get_socket_addr(&self) -> OptionSocketAddr {
        let stream_result: OptionArcRwLockStream = self.get_stream().await;
        stream_result.as_ref()?;
        let socket_addr_opt: OptionSocketAddr = stream_result
            .unwrap()
            .get_read_lock()
            .await
            .peer_addr()
            .ok();
        socket_addr_opt
    }

    /// Gets the socket address or a default if none is available.
    ///
    /// # Arguments
    ///
    /// - `&self` - A reference to the `Context`.
    ///
    /// # Returns
    ///
    /// - `SocketAddr` - The socket address or the default socket address.
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

    /// Gets the socket address as a string.
    ///
    /// # Arguments
    ///
    /// - `&self` - A reference to the `Context`.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The socket address as a string, if available.
    pub async fn get_socket_addr_string(&self) -> Option<String> {
        self.get_socket_addr().await.map(|data| data.to_string())
    }

    /// Gets the socket address as a string, or a default string if none is available.
    ///
    /// # Arguments
    ///
    /// - `&self` - A reference to the `Context`.
    ///
    /// # Returns
    ///
    /// - `String` - The socket address as a string, or the default socket address string.
    pub async fn get_socket_addr_or_default_string(&self) -> String {
        self.get_socket_addr_or_default().await.to_string()
    }

    /// Gets the socket host from the socket address.
    ///
    /// # Arguments
    ///
    /// - `&self` - A reference to the `Context`.
    ///
    /// # Returns
    ///
    /// - `OptionSocketHost` - The optional socket host.
    pub async fn get_socket_host(&self) -> OptionSocketHost {
        self.get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.ip())
    }

    /// Gets the socket port from the socket address.
    ///
    /// # Arguments
    ///
    /// - `&self` - A reference to the `Context`.
    ///
    /// # Returns
    ///
    /// - `OptionSocketPort` - The optional socket port.
    pub async fn get_socket_port(&self) -> OptionSocketPort {
        self.get_socket_addr()
            .await
            .map(|socket_addr: SocketAddr| socket_addr.port())
    }

    /// Sends data through the stream.
    ///
    /// # Arguments
    ///
    /// - `&self` - A reference to the `Context`.
    /// - `data` - The data to send, which can be converted into `ResponseData`.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - Ok(()) on success, or an error if the stream is not found or sending fails.
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

    /// Closes the stream.
    ///
    /// # Arguments
    ///
    /// - `&self` - A reference to the `Context`.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - Ok(()) on success, or an error if the stream is not found or closing fails.
    pub async fn close(&self) -> ResponseResult {
        if let Some(stream) = self.get_stream().await {
            self.get_response().await.close(&stream).await?;
            return Ok(());
        }
        Err(ResponseError::NotFoundStream)
    }

    /// Flushes the stream.
    ///
    /// # Arguments
    ///
    /// - `&self` - A reference to the `Context`.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - Ok(()) on success, or an error if the stream is not found or flushing fails.
    pub async fn flush(&self) -> ResponseResult {
        if let Some(stream) = self.get_stream().await {
            self.get_response().await.flush(&stream).await?;
            return Ok(());
        }
        Err(ResponseError::NotFoundStream)
    }

    /// Sets a data value in the context's data map.
    ///
    /// # Arguments
    ///
    /// - `&self` - A reference to the `Context`.
    /// - `key` - The key for the data.
    /// - `value` - The value to set, which must be cloneable and thread-safe.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the `Context`.
    pub async fn set_data_value<T: Any + Send + Sync + Clone>(
        &self,
        key: &str,
        value: &T,
    ) -> &Self {
        self.get_write_lock()
            .await
            .data
            .insert(key.to_owned(), Arc::new(value.clone()));
        self
    }

    /// Gets a data value from the context's data map.
    ///
    /// # Arguments
    ///
    /// - `&self` - A reference to the `Context`.
    /// - `key` - The key for the data.
    ///
    /// # Returns
    ///
    /// - `Option<T>` - The data value if found and successfully downcasted, otherwise `None`.
    pub async fn get_data_value<T: Any + Send + Sync + Clone>(&self, key: &str) -> Option<T> {
        self.get_read_lock()
            .await
            .data
            .get(key)
            .and_then(|arc| arc.downcast_ref::<T>())
            .cloned()
    }

    /// Removes a data value from the context's data map.
    ///
    /// # Arguments
    ///
    /// - `&self` - A reference to the `Context`.
    /// - `key` - The key of the data to remove.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the `Context`.
    pub async fn remove_data_value(&self, key: &str) -> &Self {
        self.get_write_lock().await.data.remove(key);
        self
    }

    /// Clears all data from the context's data map.
    ///
    /// # Arguments
    ///
    /// - `&self` - A reference to the `Context`.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to the `Context`.
    pub async fn clear_data(&self) -> &Self {
        self.get_write_lock().await.data.clear();
        self
    }
}
