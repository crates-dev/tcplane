use crate::*;

#[derive(Clone, Copy, Debug)]
struct GreetingHandler;

impl ServerHook for GreetingHandler {
    async fn new(_: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        let _ = ctx.send("Hello from GreetingHandler!").await;
    }
}

#[derive(Clone, Copy, Debug)]
struct EchoHandler;

impl ServerHook for EchoHandler {
    async fn new(_: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        let request: Request = ctx.get_request().await;
        let response: String = format!("Echo: {:?}", request);
        let _ = ctx.send(response).await;
    }
}

#[derive(Clone, Copy, Debug)]
struct PanicHandler;

impl ServerHook for PanicHandler {
    async fn new(_: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        let _ = ctx.send("Panic occurred!").await;
    }
}

#[derive(Clone, Copy, Debug)]
struct ErrorHandler;

impl ServerHook for ErrorHandler {
    async fn new(_: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        if let Some(error) = ctx.try_get_data::<String>("error").await {
            eprintln!("{error}");
            let _ = std::io::Write::flush(&mut std::io::stderr());
        }
    }
}

#[tokio::test]
async fn test_server_basic_usage() {
    let server_config: ServerConfig = ServerConfig::new();
    server_config.host("0.0.0.0").await;
    server_config.port(60000).await;
    server_config.buffer_size(100_024_000).await;

    let server: Server = Server::new();
    server.task_panic::<PanicHandler>().await;
    server.hook::<GreetingHandler>().await;
    server.hook::<EchoHandler>().await;
    server.hook::<DefaultHook>().await;
    server.read_error::<ErrorHandler>().await;

    let server_control_hook_1: ServerControlHook = server.run().await.unwrap_or_default();
    let server_control_hook_2: ServerControlHook = server_control_hook_1.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        server_control_hook_2.shutdown().await;
    });
    server_control_hook_1.wait().await;
}

#[tokio::test]
async fn test_server_handler_trait() {
    #[derive(Clone, Copy, Debug)]
    struct CustomHandler;

    impl ServerHook for CustomHandler {
        async fn new(_: &Context) -> Self {
            Self
        }

        async fn handle(self, ctx: &Context) {
            let _ = ctx.send("CustomHandler executed").await;
        }
    }

    let server: Server = Server::new();

    server.hook::<CustomHandler>().await;

    let hook_count: usize = server.read().await.get_hook().len();
    assert_eq!(hook_count, 1);
}

#[tokio::test]
async fn test_server_task_panic() {
    #[derive(Clone, Copy, Debug)]
    struct TestPanicHandler;

    impl ServerHook for TestPanicHandler {
        async fn new(_: &Context) -> Self {
            Self
        }

        async fn handle(self, _: &Context) {}
    }

    let server: Server = Server::new();

    server.task_panic::<TestPanicHandler>().await;

    let panic_count: usize = server.read().await.get_task_panic().len();
    assert_eq!(panic_count, 1);
}
