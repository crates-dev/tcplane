#[tokio::test]
async fn test_server_basic_usage() {
    use crate::*;

    fn sync_middleware(arc_lock_controller_data: ArcRwLockControllerData) {
        let mut controller_data: RwLockWriteControllerData =
            get_rw_lock_write_controller_data(&arc_lock_controller_data);
        {
            let request: &mut Vec<u8> = controller_data.get_mut_request();
            let mut new_request: Vec<u8> = request.clone();
            let ext: Vec<u8> = "test".as_bytes().to_vec();
            new_request.extend(ext);
            *request = new_request;
        }
        let request: Request = controller_data.get_request().clone();
        let stream: ArcTcpStream = controller_data.get_stream().clone().unwrap();
        let host: String = stream
            .peer_addr()
            .and_then(|host| Ok(host.to_string()))
            .unwrap_or("Unknown".to_owned());
        controller_data.get_log().debug(
            format!(
                "Request host => {}\n{:#?}\n",
                host,
                String::from_utf8_lossy(&request),
            ),
            log_debug_format_handler,
        );
    }

    async fn async_middleware(arc_lock_controller_data: ArcRwLockControllerData) {
        let controller_data: ControllerData = get_controller_data(&arc_lock_controller_data);
        println_success!(
            "async middleware request{:?}",
            String::from_utf8_lossy(controller_data.get_request())
        );
    }

    fn sync_func(arc_lock_controller_data: ArcRwLockControllerData) {
        let controller_data: ControllerData = get_controller_data(&arc_lock_controller_data);
        let stream: ArcTcpStream = controller_data.get_stream().clone().unwrap();
        let res: ResponseResult = controller_data
            .get_response()
            .clone()
            .set_data("hello world".into())
            .send(&stream);
        controller_data.get_log().debug(
            format!("Response => {:?}\n", String::from_utf8_lossy(&res.unwrap())),
            log_debug_format_handler,
        );
    }

    async fn async_func(arc_lock_controller_data: ArcRwLockControllerData) {
        let controller_data: ControllerData = get_controller_data(&arc_lock_controller_data);
        let stream: ArcTcpStream = controller_data.get_stream().clone().unwrap();
        let res: ResponseResult = controller_data
            .get_response()
            .clone()
            .set_data("Async".into())
            .send(&stream);
        controller_data.get_log().debug(
            format!("Response => {:?}\n", String::from_utf8_lossy(&res.unwrap())),
            log_debug_format_handler,
        );
    }

    async fn run_server() {
        let mut server: Server = Server::new();
        server.host("0.0.0.0");
        server.port(60000);
        server.log_dir("./logs");
        server.log_size(100_024_000);
        server.buffer(100_024_000);
        server.log_interval_millis(360);
        server.middleware(sync_middleware);
        server.async_middleware(async_middleware).await;
        server.func(sync_func);
        server.async_func(async_func).await;
        let test_string: String = "test".to_owned();
        server
            .async_func(async_func!(test_string, |data| {
                println_success!(&test_string);
                println_success!(&format!("{:?}", data));
            }))
            .await;
        server.listen();
    }

    let run_test = || {
        let mut _request_builder = RequestBuilder::new()
            .host("127.0.0.1")
            .port(60000)
            .data("hello world")
            .timeout(10000)
            .buffer(4096)
            .build();
        _request_builder
            .send()
            .and_then(|response| {
                println_success!("{:?}", response.text());
                Ok(response.binary())
            })
            .unwrap_or_default();
    };
    recoverable_spawn::r#async::recoverable_spawn(run_server);
    std::thread::sleep(std::time::Duration::from_secs(2));
    recoverable_spawn::r#sync::recoverable_spawn(run_test);
    std::thread::sleep(std::time::Duration::from_secs(4));
}
