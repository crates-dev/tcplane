use crate::*;

impl Default for Server {
    fn default() -> Self {
        Self {
            cfg: Arc::new(RwLock::new(ServerConfig::default())),
            func_list: Arc::new(RwLock::new(vec![])),
            tmp: Arc::new(RwLock::new(Tmp::default())),
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
        self.get_cfg().write().await.set_host(host.into());
        self
    }

    pub async fn port(&mut self, port: usize) -> &mut Self {
        self.get_cfg().write().await.set_port(port);
        self
    }

    pub async fn log_dir<T>(&mut self, log_dir: T) -> &mut Self
    where
        T: Into<String> + Clone,
    {
        self.get_cfg()
            .write()
            .await
            .set_log_dir(log_dir.clone().into());
        self.get_tmp()
            .write()
            .await
            .log
            .set_path(log_dir.clone().into());
        self
    }

    pub async fn log_size(&mut self, log_size: usize) -> &mut Self {
        self.get_cfg().write().await.set_log_size(log_size);
        self.get_tmp()
            .write()
            .await
            .log
            .set_limit_file_size(log_size);
        self
    }

    pub async fn enable_log(&self) -> &Self {
        self.get_cfg()
            .write()
            .await
            .set_log_size(DEFAULT_LOG_FILE_SIZE);
        self.get_tmp()
            .write()
            .await
            .get_mut_log()
            .set_limit_file_size(DEFAULT_LOG_FILE_SIZE);
        self
    }

    pub async fn disable_log(&self) -> &Self {
        self.get_cfg()
            .write()
            .await
            .set_log_size(DISABLE_LOG_FILE_SIZE);
        self.get_tmp()
            .write()
            .await
            .get_mut_log()
            .set_limit_file_size(DISABLE_LOG_FILE_SIZE);
        self
    }

    pub async fn print(&mut self, print: bool) -> &mut Self {
        self.get_cfg().write().await.set_inner_print(print);
        self
    }

    pub async fn enable_print(&mut self) -> &mut Self {
        self.print(true).await;
        self
    }

    pub async fn disable_print(&mut self) -> &mut Self {
        self.print(false).await;
        self
    }

    pub async fn open_print(&mut self, print: bool) -> &mut Self {
        self.get_cfg().write().await.set_inner_print(print);
        self
    }

    pub async fn buffer(&mut self, buffer_size: usize) -> &mut Self {
        self.get_cfg().write().await.set_buffer_size(buffer_size);
        self
    }

    pub async fn inner_print(&self, print: bool) -> &Self {
        self.get_cfg().write().await.set_inner_print(print);
        self
    }

    pub async fn inner_log(&self, print: bool) -> &Self {
        self.get_cfg().write().await.set_inner_log(print);
        self
    }

    pub async fn enable_inner_print(&self) -> &Self {
        self.inner_print(true).await;
        self
    }

    pub async fn disable_inner_print(&self) -> &Self {
        self.inner_print(false).await;
        self
    }

    pub async fn enable_inner_log(&self) -> &Self {
        self.inner_log(true).await;
        self
    }

    pub async fn disable_inner_log(&self) -> &Self {
        self.inner_log(false).await;
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
            .push(Box::new(move |controller_data| {
                Box::pin(func(controller_data))
            }));
        self
    }

    pub(super) async fn handle_stream(cfg: &ServerConfig, stream_lock: ArcRwLockStream) -> Vec<u8> {
        let buffer_size: usize = cfg.get_buffer_size().clone().max(SPLIT_REQUEST_BYTES.len());
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

    pub async fn listen(&mut self) -> &mut Self {
        self.init().await;
        let cfg: ServerConfig = self.get_cfg().read().await.clone();
        let host: String = cfg.get_host().to_owned();
        let port: usize = *cfg.get_port();
        let addr: String = format!("{}{}{}", host, COLON_SPACE_SYMBOL, port);
        let tcp_listener: TcpListener = TcpListener::bind(&addr)
            .await
            .map_err(|e| ServerError::TcpBindError(e.to_string()))
            .unwrap();
        while let Ok((stream, _)) = tcp_listener.accept().await {
            let tmp_arc_lock: ArcRwLockTmp = Arc::clone(&self.tmp);
            let stream_lock: ArcRwLockStream = ArcRwLockStream::from_stream(stream);
            let func_list_arc_lock: ArcRwlockVecBoxFunc = Arc::clone(&self.get_func_list());
            let cfg_arc_lock: ArcRwLockServerConfig = Arc::clone(&self.get_cfg());
            let handle_request = move || async move {
                let cfg: ServerConfig = cfg_arc_lock.read().await.clone();
                let request: Vec<u8> = Self::handle_stream(&cfg, stream_lock.clone()).await;
                let log: Log = tmp_arc_lock.read().await.get_log().clone();
                let mut controller_data: InnerControllerData = InnerControllerData::new();
                controller_data
                    .set_stream(Some(stream_lock.clone()))
                    .set_request(request)
                    .set_log(log);
                let controller_data: ControllerData =
                    ControllerData::from_controller_data(controller_data);
                for func in func_list_arc_lock.read().await.iter() {
                    func(controller_data.clone()).await;
                }
            };
            tokio::spawn(handle_request());
        }
        self
    }

    async fn init_panic_hook(&self) {
        let tmp: Tmp = self.get_tmp().read().await.clone();
        let cfg: ServerConfig = self.get_cfg().read().await.clone();
        let enable_inner_print: bool = *cfg.get_inner_print();
        let enable_inner_log: bool = *cfg.get_inner_log() && tmp.get_log().is_enable();
        set_hook(Box::new(move |err| {
            let err_string: String = err.to_string();
            if enable_inner_print {
                println_error!(err_string);
            }
            if enable_inner_log {
                handle_error(&tmp, &err_string);
            }
        }));
    }

    async fn init(&self) {
        self.init_panic_hook().await;
    }
}
