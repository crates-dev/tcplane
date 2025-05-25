use crate::*;

impl Default for Server {
    fn default() -> Self {
        Self {
            config: Arc::new(RwLock::new(ServerConfig::default())),
            func_list: Arc::new(RwLock::new(vec![])),
        }
    }
}

impl Server {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn host<T>(&mut self, host: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.get_config().write().await.set_host(host.into());
        self
    }

    pub async fn port(&mut self, port: usize) -> &mut Self {
        self.get_config().write().await.set_port(port);
        self
    }

    pub async fn buffer(&mut self, buffer_size: usize) -> &mut Self {
        self.get_config().write().await.set_buffer_size(buffer_size);
        self
    }

    pub async fn error_handle<F>(&self, func: F) -> &Self
    where
        F: ErrorHandle + Send + Sync + 'static,
    {
        self.get_config()
            .write()
            .await
            .set_error_handle(Arc::new(func));
        self
    }

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

    pub(super) async fn handle_stream(
        config: &ServerConfig,
        stream_lock: ArcRwLockStream,
    ) -> Vec<u8> {
        let buffer_size: usize = config
            .get_buffer_size()
            .clone()
            .max(SPLIT_REQUEST_BYTES.len());
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

    pub async fn run(&mut self) -> &mut Self {
        self.init().await;
        let config: ServerConfig = self.get_config().read().await.clone();
        let host: String = config.get_host().to_owned();
        let port: usize = *config.get_port();
        let addr: String = format!("{}{}{}", host, COLON_SPACE_SYMBOL, port);
        let tcp_listener: TcpListener = TcpListener::bind(&addr)
            .await
            .map_err(|e| ServerError::TcpBindError(e.to_string()))
            .unwrap();
        while let Ok((stream, _)) = tcp_listener.accept().await {
            let stream_lock: ArcRwLockStream = ArcRwLockStream::from_stream(stream);
            let func_list_arc_lock: ArcRwlockVecBoxFunc = Arc::clone(&self.get_func_list());
            let config_arc_lock: ArcRwLockServerConfig = Arc::clone(&self.get_config());
            let handle_request = move || async move {
                let config: ServerConfig = config_arc_lock.read().await.clone();
                let request: Vec<u8> = Self::handle_stream(&config, stream_lock.clone()).await;
                let mut ctx: InnerContext = InnerContext::new();
                ctx.set_stream(Some(stream_lock.clone()))
                    .set_request(request);
                let ctx: Context = Context::from_inner_context(ctx);
                for func in func_list_arc_lock.read().await.iter() {
                    func(ctx.clone()).await;
                }
            };
            tokio::spawn(handle_request());
        }
        self
    }

    async fn init_panic_hook(&self) {
        let error_handle: ArcErrorHandle =
            self.get_config().read().await.get_error_handle().clone();
        set_hook(Box::new(move |err| {
            let data: String = err.to_string();
            error_handle(data);
        }));
    }

    async fn init(&self) {
        self.init_panic_hook().await;
    }
}
