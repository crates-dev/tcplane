<center>

## tcplane

[![](https://img.shields.io/crates/v/tcplane.svg)](https://crates.io/crates/tcplane)
[![](https://img.shields.io/crates/d/tcplane.svg)](https://img.shields.io/crates/d/tcplane.svg)
[![](https://docs.rs/tcplane/badge.svg)](https://docs.rs/tcplane)
[![](https://github.com/crates-dev/tcplane/workflows/Rust/badge.svg)](https://github.com/crates-dev/tcplane/actions?query=workflow:Rust)
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
    ctx.send("tcplane: 1").await.unwrap();
}

fn error_handle(error: String) {
    eprintln!("{}", error);
    let _ = std::io::Write::flush(&mut std::io::stderr());
}

#[tokio::main]
async fn main() {
    let mut server: Server = Server::new().await;
    server.host("0.0.0.0").await;
    server.port(60000).await;
    server.buffer(100_024_000).await;
    server.error_handle(error_handle).await;
    server.func(test_func).await;
    server
        .func(|ctx: Context| async move {
            ctx.send("tcplane: 2").await.unwrap();
        })
        .await;
    server.run().await;
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
