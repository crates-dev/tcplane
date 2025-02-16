use crate::*;
use http_type::*;

impl Default for Server {
    #[inline]
    fn default() -> Self {
        Self {
            cfg: Arc::new(RwLock::new(ServerConfig::default())),
            func: Arc::new(RwLock::new(Box::new(
                |_controller_data: ArcRwLockControllerData| {},
            ))),
            middleware: Arc::new(RwLock::new(vec![])),
            async_middleware: Arc::new(tokio::sync::RwLock::new(vec![])),
            async_func: Arc::new(tokio::sync::RwLock::new(Box::new(
                |_controller_data: ArcRwLockControllerData| Box::pin(async {}),
            ))),
            tmp: Arc::new(RwLock::new(Tmp::default())),
        }
    }
}

impl Server {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn host<T>(&mut self, host: T) -> &mut Self
    where
        T: Into<String>,
    {
        let _ = self.get_cfg().write().and_then(|mut cfg| {
            cfg.set_host(host.into());
            Ok(())
        });
        self
    }

    #[inline]
    pub fn port(&mut self, port: usize) -> &mut Self {
        let _ = self.get_cfg().write().and_then(|mut cfg| {
            cfg.set_port(port);
            Ok(())
        });
        self
    }

    #[inline]
    pub fn log_dir<T>(&mut self, log_dir: T) -> &mut Self
    where
        T: Into<String> + Clone,
    {
        let _ = self.get_cfg().write().and_then(|mut cfg| {
            cfg.set_log_dir(log_dir.clone().into());
            Ok(())
        });
        let _ = self.get_tmp().write().and_then(|mut tmp| {
            tmp.log.set_path(log_dir.clone().into());
            Ok(())
        });
        self
    }

    #[inline]
    pub fn log_size(&mut self, log_size: usize) -> &mut Self {
        let _ = self.get_cfg().write().and_then(|mut cfg| {
            cfg.set_log_size(log_size);
            Ok(())
        });
        let _ = self.get_tmp().write().and_then(|mut tmp| {
            tmp.log.set_file_size(log_size);
            Ok(())
        });
        self
    }

    #[inline]
    pub fn log_interval_millis(&mut self, interval_millis: usize) -> &mut Self {
        let _ = self.get_cfg().write().and_then(|mut cfg| {
            cfg.set_interval_millis(interval_millis);
            Ok(())
        });
        let _ = self.get_tmp().write().and_then(|mut tmp| {
            tmp.log.set_interval_millis(interval_millis);
            Ok(())
        });
        self
    }

    #[inline]
    pub fn print(&mut self, print: bool) -> &mut Self {
        let _ = self.get_cfg().write().and_then(|mut cfg| {
            cfg.set_print(print);
            Ok(())
        });
        self
    }

    #[inline]
    pub fn enable_print(&mut self) -> &mut Self {
        self.print(true);
        self
    }

    #[inline]
    pub fn disable_print(&mut self) -> &mut Self {
        self.print(false);
        self
    }

    #[inline]
    pub fn open_print(&mut self, print: bool) -> &mut Self {
        let _ = self.get_cfg().write().and_then(|mut cfg| {
            cfg.set_print(print);
            Ok(())
        });
        self
    }

    #[inline]
    pub fn buffer(&mut self, buffer_size: usize) -> &mut Self {
        let _ = self.get_cfg().write().and_then(|mut cfg| {
            cfg.set_buffer_size(buffer_size);
            Ok(())
        });
        self
    }

    #[inline]
    pub fn func<F>(&mut self, func: F) -> &mut Self
    where
        F: 'static + Send + Sync + Fn(ArcRwLockControllerData),
    {
        if let Ok(mut mut_func) = self.func.write() {
            *mut_func = Box::new(func);
        }
        self
    }

    #[inline]
    pub fn middleware<F>(&mut self, func: F) -> &mut Self
    where
        F: 'static + Send + Sync + Fn(ArcRwLockControllerData),
    {
        if let Ok(mut middleware) = self.middleware.write() {
            middleware.push(Box::new(func));
        }
        self
    }

    #[inline]
    pub async fn async_func<F, Fut>(&mut self, func: F) -> &mut Self
    where
        F: AsyncFuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        {
            let mut mut_async_func: tokio::sync::RwLockWriteGuard<'_, Box<dyn AsyncFunc>> =
                self.async_func.write().await;
            *mut_async_func =
                Box::new(move |arc_lock_controller_data| Box::pin(func(arc_lock_controller_data)));
        }
        self
    }

    #[inline]
    pub async fn async_middleware<F, Fut>(&mut self, func: F) -> &mut Self
    where
        F: AsyncFuncWithoutPin<Fut>,
        Fut: Future<Output = ()> + Send + Sync + 'static,
    {
        {
            let mut mut_async_middleware: tokio::sync::RwLockWriteGuard<
                '_,
                Vec<Box<dyn AsyncFunc>>,
            > = self.async_middleware.write().await;
            mut_async_middleware.push(Box::new(move |arc_lock_controller_data| {
                Box::pin(func(arc_lock_controller_data))
            }));
        }
        self
    }

    #[inline]
    fn handle_stream(&self, mut stream: &TcpStream) -> Vec<u8> {
        let buffer_size: usize = self
            .get_cfg()
            .read()
            .and_then(|cfg| Ok(cfg.get_buffer_size().clone()))
            .unwrap_or_default()
            .max(HTTP_DOUBLE_BR_BYTES.len());
        let mut buffer: Vec<u8> = Vec::new();
        let mut tmp_buf: Vec<u8> = vec![0u8; buffer_size];
        loop {
            match stream.read(&mut tmp_buf) {
                Ok(n) => {
                    let old_len: usize = tmp_buf.len();
                    tmp_buf = remove_trailing_zeros(&mut tmp_buf);
                    let new_len: usize = tmp_buf.len();
                    if n == 0 {
                        break;
                    }
                    if old_len != new_len || tmp_buf.ends_with(HTTP_DOUBLE_BR_BYTES) {
                        buffer.extend_from_slice(&tmp_buf[..n - HTTP_DOUBLE_BR_BYTES.len()]);
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

    #[inline]
    pub fn listen(&mut self) -> &mut Self {
        self.init();
        let mut host: String = EMPTY_STR.to_owned();
        let mut port: usize = usize::default();
        let _ = self.get_cfg().read().and_then(|cfg| {
            host = cfg.get_host().to_owned();
            port = *cfg.get_port();
            Ok(())
        });
        let addr: String = format!("{}{}{}", host, COLON_SPACE_SYMBOL, port);
        let listener_res: Result<TcpListener, ServerError> =
            TcpListener::bind(&addr).map_err(|e| ServerError::TcpBindError(e.to_string()));
        if let Err(err) = listener_res {
            let _ = self.get_tmp().write().and_then(|tmp| {
                tmp.get_log().error(err.to_string(), common_log);
                Ok(())
            });
            return self;
        }
        let tcp_listener: TcpListener = listener_res.unwrap();
        for stream_res in tcp_listener.incoming() {
            let tmp_arc_lock: ArcRwLock<Tmp> = Arc::clone(&self.tmp);
            if let Err(err) = stream_res {
                use recoverable_spawn::sync::*;
                let _ = run_function(move || {
                    if let Ok(tem) = tmp_arc_lock.read() {
                        tem.get_log().error(err.to_string(), common_log);
                    }
                });
                continue;
            }
            let stream: TcpStream = stream_res.unwrap();
            let stream_arc: Arc<TcpStream> = Arc::new(stream);
            let middleware_arc_lock: MiddlewareArcLock = Arc::clone(&self.middleware);
            let async_middleware_arc_lock: AsyncMiddlewareArcLock =
                Arc::clone(&self.async_middleware);
            let func_arc_lock: FuncArcLock = Arc::clone(&self.func);
            let async_func_arc_lock: AsyncFuncArcLock = Arc::clone(&self.async_func);
            let request: Vec<u8> = self.handle_stream(&stream_arc);
            let handle_request = move || async move {
                let log: Log = tmp_arc_lock
                    .read()
                    .and_then(|tmp| Ok(tmp.log.clone()))
                    .unwrap_or_default();
                let mut controller_data: ControllerData = ControllerData::new();
                controller_data
                    .set_stream(Some(stream_arc))
                    .set_request(request)
                    .set_log(log);
                let arc_lock_controller_data: ArcRwLockControllerData =
                    Arc::new(RwLock::new(controller_data));
                if let Ok(middleware_guard) = middleware_arc_lock.read() {
                    for middleware in middleware_guard.iter() {
                        middleware(arc_lock_controller_data.clone());
                    }
                }
                for async_middleware in async_middleware_arc_lock.read().await.iter() {
                    async_middleware(arc_lock_controller_data.clone()).await;
                }
                if let Ok(func_guard) = func_arc_lock.read() {
                    func_guard(arc_lock_controller_data.clone());
                }
                async_func_arc_lock.read().await(arc_lock_controller_data.clone()).await;
            };
            tokio::spawn(async move {
                handle_request().await;
            });
        }
        self
    }

    #[inline]
    fn init_panic_hook(&self) {
        let tmp: Tmp = self
            .tmp
            .read()
            .map(|tmp| tmp.clone())
            .unwrap_or_else(|_| Tmp::default());
        let print: bool = self
            .get_cfg()
            .read()
            .and_then(|cfg| Ok(cfg.get_print().clone()))
            .unwrap_or(DEFAULT_PRINT);
        set_hook(Box::new(move |err| {
            let err_msg: String = format!("{}", err);
            if print {
                println_error!(err_msg);
            }
            handle_error(&tmp, err_msg.clone());
        }));
    }

    #[inline]
    fn init_log(&self) {
        let _ = self.get_tmp().read().and_then(|tmp| {
            log_run(tmp.get_log());
            Ok(())
        });
    }

    #[inline]
    fn init(&self) {
        self.init_panic_hook();
        self.init_log();
    }
}
