use crate::*;

/// Default implementation for Server.
impl Default for Server {
    fn default() -> Self {
        Self {
            config: Arc::new(RwLock::new(ServerConfig::default())),
            func_list: Arc::new(RwLock::new(vec![])),
        }
    }
}

/// Server implementation containing all server operations.
impl Server {
    /// Creates a new Server instance with default configuration.
    ///
    /// # Returns
    ///
    /// - `Server` - New server instance with default settings.
    pub async fn new() -> Self {
        Self::default()
    }

    /// Sets the server host address.
    ///
    /// # Arguments
    ///
    /// - `T` - Type that can be converted into String (host address)
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference to self for method chaining.
    pub async fn host<T>(&mut self, host: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.config.write().await.host = host.into();
        self
    }

    /// Sets the server listening port.
    ///
    /// # Arguments
    ///
    /// - `usize` - Port number to listen on
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference to self for method chaining.
    pub async fn port(&mut self, port: usize) -> &mut Self {
        self.config.write().await.port = port;
        self
    }

    /// Sets the network buffer size for the server.
    ///
    /// # Arguments
    ///
    /// - `usize` - Buffer size in bytes
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference to self for method chaining.
    pub async fn buffer(&mut self, buffer_size: usize) -> &mut Self {
        self.config.write().await.buffer_size = buffer_size;
        self
    }

    /// Sets the error handler for the server.
    ///
    /// # Arguments
    ///
    /// - `F` - Error handler function implementing ErrorHandle trait
    ///
    /// # Returns
    ///
    /// - `&Self` - Immutable reference to self for method chaining.
    pub async fn error_handle<F>(&self, func: F) -> &Self
    where
        F: ErrorHandle + Send + Sync + 'static,
    {
        self.config.write().await.error_handle = Arc::new(func);
        self
    }

    /// Adds an async function to the server's handler list.
    ///
    /// # Arguments
    ///
    /// - `F` - Async function type implementing AsyncFuncWithoutPin trait
    /// - `Fut` - Future type returned by the async function
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference to self for method chaining.
    pub async fn func<F, Fut>(&mut self, func: F) -> &mut Self
    where
        F: AsyncFuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        self.func_list
            .write()
            .await
            .push(Box::new(move |ctx| Box::pin(func(ctx))));
        self
    }

    /// Handles incoming TCP stream and reads data into buffer.
    ///
    /// # Arguments
    ///
    /// - `&ServerConfig` - Server configuration reference
    /// - `ArcRwLockStream` - Thread-safe TCP stream wrapper
    ///
    /// # Returns
    ///
    /// - `Vec<u8>` - Byte buffer containing the received data
    pub(super) async fn handle_stream(
        config: &ServerConfig,
        stream_lock: ArcRwLockStream,
    ) -> Vec<u8> {
        let buffer_size: usize = config.buffer_size.max(SPLIT_REQUEST_BYTES.len());
        let mut buffer: Vec<u8> = Vec::new();
        let mut tmp_buf: Vec<u8> = vec![0u8; buffer_size];
        let mut stream: RwLockWriteGuard<'_, TcpStream> = stream_lock.get_write_lock().await;
        loop {
            match stream.read(&mut tmp_buf).await {
                Ok(n) => {
                    let old_len: usize = tmp_buf.len();
                    tmp_buf = remove_trailing_zeros(&mut tmp_buf);
                    let new_len: usize = tmp_buf.len();
                    if n == 0 {
                        break;
                    }
                    if old_len != new_len || tmp_buf.ends_with(SPLIT_REQUEST_BYTES) {
                        buffer.extend_from_slice(&tmp_buf[..n - SPLIT_REQUEST_BYTES.len()]);
                        break;
                    }
                    buffer.extend_from_slice(&tmp_buf[..n]);
                }
                _ => {
                    break;
                }
            }
        }
        buffer
    }

    /// Starts the server and begins accepting connections.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference to self for method chaining.
    pub async fn run(&mut self) -> &mut Self {
        self.init().await;
        let config: ServerConfig = self.config.read().await.clone();
        let host: String = config.host.to_owned();
        let port: usize = config.port;
        let addr: String = format!("{}{}{}", host, COLON_SPACE_SYMBOL, port);
        let tcp_listener: TcpListener = TcpListener::bind(&addr)
            .await
            .map_err(|e| ServerError::TcpBindError(e.to_string()))
            .unwrap();
        while let Ok((stream, _)) = tcp_listener.accept().await {
            let stream_lock: ArcRwLockStream = ArcRwLockStream::from_stream(stream);
            let func_list_arc_lock: ArcRwlockVecBoxFunc = Arc::clone(&self.func_list);
            let config_arc_lock: ArcRwLockServerConfig = Arc::clone(&self.config);
            let handle_request = move || async move {
                let config: ServerConfig = config_arc_lock.read().await.clone();
                let request: Vec<u8> = Self::handle_stream(&config, stream_lock.clone()).await;
                let mut ctx: InnerContext = InnerContext::new();
                ctx.stream = Some(stream_lock.clone());
                ctx.request = request;
                let ctx: Context = Context::from_inner_context(ctx);
                for func in func_list_arc_lock.read().await.iter() {
                    func(ctx.clone()).await;
                }
            };
            tokio::spawn(handle_request());
        }
        self
    }

    /// Initializes the panic hook to use the configured error handler.
    ///
    /// # Returns
    ///
    /// - `()` - This function does not return any meaningful value.
    async fn init_panic_hook(&self) {
        let error_handle: ArcErrorHandle = self.config.read().await.error_handle.clone();
        set_hook(Box::new(move |err| {
            let data: String = err.to_string();
            error_handle(data);
        }));
    }

    /// Initializes server components including panic hook.
    ///
    /// # Returns
    ///
    /// - `()` - This function does not return any meaningful value.
    async fn init(&self) {
        self.init_panic_hook().await;
    }
}
