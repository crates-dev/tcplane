## tcplane

[![](https://img.shields.io/crates/v/tcplane.svg)](https://crates.io/crates/tcplane)
[![](https://docs.rs/tcplane/badge.svg)](https://docs.rs/tcplane)
[![](https://img.shields.io/crates/l/tcplane.svg)](./LICENSE)
[![](https://github.com/ltpp-universe/tcplane/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/tcplane/actions?query=workflow:Rust)

[Official Documentation](https://docs.ltpp.vip/tcplane/)

[Api Docs](https://docs.rs/tcplane/latest/tcplane/)

> tcplane is a lightweight and high-performance Rust TCP server library designed to simplify network service development. It supports TCP communication, data stream management, and connection handling, focusing on providing efficient low-level network connections and data transmission capabilities, making it ideal for building modern network services.

## Installation

To use this crate, you can run cmd:

```shell
cargo add tcplane
```

## Use

### Sync

```rust
use tcplane::*;

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

fn run_server() {
    let mut server: Server = Server::new();
    server.host("0.0.0.0");
    server.port(60000);
    server.thread_pool_size(10);
    server.log_dir("./logs");
    server.log_size(1_024_000);
    server.buffer(1_024_000);
    server.middleware(|arc_lock_controller_data| {
        let mut controller_data: RwLockWriteControllerData = arc_lock_controller_data.write().unwrap();
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

        controller_data.get_log().log_debug(
            format!(
                "Request host => {}\n{:#?}\n",
                host,
                String::from_utf8_lossy(&request),
            ),
            common_log,
        );
    });

    server.func(|arc_lock_controller_data| {
        let controller_data: RwLockWriteControllerData = arc_lock_controller_data.write().unwrap();
        let stream: ArcTcpStream = controller_data.get_stream().clone().unwrap();
        let res: ResponseResult = controller_data
            .get_response()
            .clone()
            .data("hello world")
            .send(&stream);
        controller_data.get_log().log_debug(
            format!("Response => {:?}\n", String::from_utf8_lossy(&res.unwrap())),
            common_log,
        );
    });
    server.listen();
}
```

### Async

```rust
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

async fn run_server() {
    let mut server: Server = Server::new();
    server.host("0.0.0.0");
    server.port(60000);
    server.thread_pool_size(10);
    server.log_dir("./logs");
    server.log_size(1_024_000);
    server.buffer(1_024_000);
    server.middleware(|arc_lock_controller_data| {
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

        controller_data.get_log().log_debug(
            format!(
                "Request host => {}\n{:#?}\n",
                host,
                String::from_utf8_lossy(&request),
            ),
            common_log,
        );
    });

    server
        .async_middleware(|arc_lock_controller_data| async move {
            let controller_data: RwLockWriteControllerData =
                arc_lock_controller_data.write().unwrap();
            println!(
                "async middleware request{:?}",
                String::from_utf8_lossy(controller_data.get_request())
            );
        })
        .await;

    server.func(|arc_lock_controller_data| {
        let controller_data: RwLockWriteControllerData =
            arc_lock_controller_data.write().unwrap();
        let stream: ArcTcpStream = controller_data.get_stream().clone().unwrap();
        let res: ResponseResult = controller_data
            .get_response()
            .clone()
            .data("hello world")
            .send(&stream);
        controller_data.get_log().log_debug(
            format!("Response => {:?}\n", String::from_utf8_lossy(&res.unwrap())),
            common_log,
        );
    });
    server.listen();
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
