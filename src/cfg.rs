#[tokio::test]
async fn test_server_basic_usage() {
    use crate::*;

    async fn test_middleware(arc_lock_controller_data: ArcRwLockControllerData) {
        let mut controller_data: RwLockWriteControllerData =
            arc_lock_controller_data.get_write_lock().await;
        {
            let request: &mut Request = controller_data.get_mut_request();
            let mut new_request: Request = request.clone();
            let ext: Request = "hello world".as_bytes().to_vec();
            new_request.extend(ext);
            *request = new_request;
        }
        let request: Request = controller_data.get_request().clone();
        let host: String = arc_lock_controller_data.get_socket_addr().await.unwrap();
        controller_data.get_log().debug(
            format!(
                "Request host => {}\n{:#?}\n",
                host,
                String::from_utf8_lossy(&request),
            ),
            log_debug_format_handler,
        );
    }

    async fn test_func(arc_lock_controller_data: ArcRwLockControllerData) {
        let res: ResponseData = arc_lock_controller_data.send("tcplane").await.unwrap();
        arc_lock_controller_data.get_controller_data().await.get_log().debug(
            format!("Response => {:?}\n", String::from_utf8_lossy(&res)),
            log_debug_format_handler,
        );
    }

    async fn run_server() {
        let mut server: Server = Server::new();
        server.host("0.0.0.0").await;
        server.port(60000).await;
        server.log_dir("./logs").await;
        server.log_size(100_024_000).await;
        server.buffer(100_024_000).await;
        server.log_interval_millis(360).await;
        server.middleware(test_middleware).await;
        server.func(test_func).await;
        let test_string: String = "test".to_owned();
        server
            .func(async_func!(test_string, |data| {
                println_success!(&test_string);
                println_success!(&format!("{:?}", data));
            }))
            .await;
        server.listen().await;
    }

    recoverable_spawn::r#async::recoverable_spawn(run_server);
    std::thread::sleep(std::time::Duration::from_secs(10));
}
