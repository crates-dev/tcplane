#[tokio::test]
async fn test_server_basic_usage() {
    use crate::*;

    fn println(data: &str) {
        let binding: String = current_time();
        let mut time_output_builder: OutputBuilder<'_> = OutputBuilder::new();
        let mut text_output_builder: OutputBuilder<'_> = OutputBuilder::new();
        let time_output: Output<'_> = time_output_builder
            .text(&binding)
            .blod(true)
            .bg_color(ColorType::Use(Color::Yellow))
            .color(ColorType::Rgb(255, 255, 255))
            .build();
        let text_output: Output<'_> = text_output_builder
            .text(data)
            .blod(true)
            .bg_color(ColorType::Use(Color::Green))
            .color(ColorType::Rgb(255, 255, 255))
            .endl(true)
            .build();
        OutputListBuilder::new()
            .add(time_output)
            .add(text_output)
            .run();
    }

    fn common_log(log_data: &String) -> String {
        println(&log_data);
        let write_data: String = format!("{}: {}\n", current_time(), log_data);
        write_data.clone()
    }

    fn sync_middleware(arc_lock_controller_data: ArcRwLockControllerData) {
        let mut controller_data: RwLockWriteControllerData =
            arc_lock_controller_data.write().unwrap();
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
            common_log,
        );
    }

    async fn async_middleware(arc_lock_controller_data: ArcRwLockControllerData) {
        let controller_data: RwLockWriteControllerData = arc_lock_controller_data.write().unwrap();
        println!(
            "async middleware request{:?}",
            String::from_utf8_lossy(controller_data.get_request())
        );
    }

    fn sync_func(arc_lock_controller_data: ArcRwLockControllerData) {
        let controller_data: RwLockWriteControllerData = arc_lock_controller_data.write().unwrap();
        let stream: ArcTcpStream = controller_data.get_stream().clone().unwrap();
        let res: ResponseResult = controller_data
            .get_response()
            .clone()
            .set_data("hello world".into())
            .send(&stream);
        controller_data.get_log().debug(
            format!("Response => {:?}\n", String::from_utf8_lossy(&res.unwrap())),
            common_log,
        );
    }

    async fn async_func(arc_lock_controller_data: ArcRwLockControllerData) {
        let controller_data: RwLockWriteControllerData = arc_lock_controller_data.write().unwrap();
        let stream: ArcTcpStream = controller_data.get_stream().clone().unwrap();
        let res: ResponseResult = controller_data
            .get_response()
            .clone()
            .set_data("Async".into())
            .send(&stream);
        controller_data.get_log().debug(
            format!("Response => {:?}\n", String::from_utf8_lossy(&res.unwrap())),
            common_log,
        );
    }

    async fn run_server() {
        let mut server: Server = Server::new();
        server.host("0.0.0.0");
        server.port(60000);
        server.thread_pool_size(10);
        server.log_dir("./logs");
        server.log_size(1_024_000);
        server.buffer(1_024_000);
        server.middleware(sync_middleware);
        server.async_middleware(async_middleware).await;
        server.func(sync_func);
        server.async_func(async_func).await;
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
                println!("{:?}", response.text());
                Ok(response.binary())
            })
            .unwrap_or_default();
    };
    async_recoverable_spawn(run_server);
    std::thread::sleep(std::time::Duration::from_secs(2));
    recoverable_spawn(run_test);
    std::thread::sleep(std::time::Duration::from_secs(4));
}
