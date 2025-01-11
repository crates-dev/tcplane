use super::{
    config::r#type::*, controller_data::r#type::*, error::r#type::*, func::r#type::*,
    middleware::r#type::*, r#type::*, tmp::r#type::*,
};
use crate::utils::list::*;
use crate::*;
use http_type::*;
use recoverable_thread_pool::*;
use std::io::Read;

impl Default for Server {
    #[inline]
    fn default() -> Self {
        Self {
            cfg: Arc::new(RwLock::new(ServerConfig::default())),
            func: Arc::new(RwLock::new(Box::new(|_| {}))),
            middleware: Arc::new(RwLock::new(vec![])),
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
    pub fn thread_pool_size(&mut self, thread_pool_size: usize) -> &mut Self {
        let _ = self.get_cfg().write().and_then(|mut cfg| {
            cfg.set_thread_pool_size(thread_pool_size);
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
        F: 'static + Send + Sync + Fn(&mut ControllerData),
    {
        if let Ok(mut mut_func) = self.func.write() {
            *mut_func = Box::new(func);
        }
        self
    }

    #[inline]
    pub fn middleware<F>(&mut self, func: F) -> &mut Self
    where
        F: 'static + Send + Sync + Fn(&mut ControllerData),
    {
        if let Ok(mut middleware) = self.middleware.write() {
            middleware.push(Box::new(func));
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
    fn common_log(data: &String) -> String {
        format!("{}: {}{}", current_time(), data.to_string(), HTTP_BR)
    }

    #[inline]
    pub fn listen(&mut self) -> &mut Self {
        self.init();
        let mut host: String = EMPTY_STR.to_owned();
        let mut port: usize = usize::default();
        let mut thread_pool_size: usize = usize::default();
        let _ = self.get_cfg().read().and_then(|cfg| {
            host = cfg.get_host().to_owned();
            port = *cfg.get_port();
            thread_pool_size = *cfg.get_thread_pool_size();
            Ok(())
        });
        let addr: String = format!("{}{}{}", host, COLON_SPACE_SYMBOL, port);
        let listener_res: Result<TcpListener, Error> =
            TcpListener::bind(&addr).map_err(|e| Error::TcpBindError(e.to_string()));
        if listener_res.is_err() {
            let _ = self.get_tmp().write().and_then(|tmp| {
                tmp.get_log().log_error(
                    format!("{}", listener_res.err().unwrap_or(Error::Unknown)),
                    Self::common_log,
                );
                Ok(())
            });
            return self;
        }
        let tcp_listener: TcpListener = listener_res.unwrap();
        let thread_pool: ThreadPool = ThreadPool::new(thread_pool_size);
        for stream_res in tcp_listener.incoming() {
            if stream_res.is_err() {
                continue;
            }
            let stream: TcpStream = stream_res.unwrap();
            let stream_arc: Arc<TcpStream> = Arc::new(stream);
            let middleware_arc: MiddlewareArcLock = Arc::clone(&self.middleware);
            let func: FuncArcLock = Arc::clone(&self.func);
            let thread_pool_func_tmp_arc: ArcRwLock<Tmp> = Arc::clone(&self.tmp);
            let error_handle_tmp_arc: ArcRwLock<Tmp> = Arc::clone(&self.tmp);
            let request: Vec<u8> = self.handle_stream(&stream_arc);
            let thread_pool_func = move || {
                let log: Log = thread_pool_func_tmp_arc
                    .read()
                    .and_then(|tmp| Ok(tmp.log.clone()))
                    .unwrap_or_default();
                let mut controller_data: ControllerData = ControllerData::new();
                controller_data
                    .set_stream(Some(stream_arc.clone()))
                    .set_response(super::response::r#type::Response { data: vec![] })
                    .set_request(request)
                    .set_log(log);
                if let Ok(middleware_guard) = middleware_arc.read() {
                    for middleware in middleware_guard.iter() {
                        middleware(&mut controller_data);
                    }
                }
                if let Ok(func_guard) = func.read() {
                    func_guard(&mut controller_data);
                }
            };
            let handle_error_func = move |err_str: &str| {
                let err_string: String = err_str.to_owned();
                if let Ok(tem) = error_handle_tmp_arc.read() {
                    tem.get_log().log_error(err_string, Self::common_log);
                }
            };
            let _ = thread_pool.execute(thread_pool_func, handle_error_func);
        }
        self
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
        self.init_log();
    }
}
