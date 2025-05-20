<center>

## tcplane

[![](https://img.shields.io/crates/v/tcplane.svg)](https://crates.io/crates/tcplane)
[![](https://img.shields.io/crates/d/tcplane.svg)](https://img.shields.io/crates/d/tcplane.svg)
[![](https://docs.rs/tcplane/badge.svg)](https://docs.rs/tcplane)
[![](https://github.com/eastspire/tcplane/workflows/Rust/badge.svg)](https://github.com/eastspire/tcplane/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/tcplane.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/tcplane/)

[Api Docs](https://docs.rs/tcplane/latest/tcplane/)

> tcplane is a lightweight and high-performance Rust TCP server library designed to simplify network service development. It supports TCP communication, data stream management, and connection handling, focusing on providing efficient low-level network connections and data transmission capabilities, making it ideal for building modern network services.

## Installation

To use this crate, you can run cmd:

```shell
cargo add tcplane
```

## Use

```rust
use tcplane::*;

async fn test_func(ctx: Context) {
    ctx.send("tcplane").await.unwrap();
    let response: Response = ctx.get_response().await;
    let response_data: &ResponseData = response.get_response_data();
    ctx.log_debug(
        &format!(
            "Response => {:?}\n",
            String::from_utf8_lossy(&response_data)
        ),
        log_handler,
    )
    .await;
}

#[tokio::main]
async fn main() {
    let mut server: Server = Server::new();
    server.host("0.0.0.0").await;
    server.port(60000).await;
    server.log_dir("./logs").await;
    server.log_size(100_024_000).await;
    server.buffer(100_024_000).await;
    server.func(test_func).await;
    let test_string: String = "test".to_owned();
    server
        .func(future_fn!(test_string, |data: Context| {
            println_success!(&test_string);
            println_success!(String::from_utf8_lossy(&data.get_request().await));
        }))
        .await;
    server.run().await;
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [eastspire <root@ltpp.vip>](mailto:root@ltpp.vip).
