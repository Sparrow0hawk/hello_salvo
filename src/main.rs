use salvo::prelude::*;

#[handler]
async fn hello() -> &'static str {
    "Hello world"
}

pub fn route() -> Router {
    Router::new().get(hello)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let acceptor = TcpListener::new("127.0.0.1:8080").bind().await;
    Server::new(acceptor).serve(route()).await;
}

#[cfg(test)]
mod tests {
    use salvo::prelude::*;
    use salvo::test::{ResponseExt, TestClient};
    use super::{route};

    #[tokio::test]
    async fn test_hello_world() {
        let service = Service::new(route());

        let content = TestClient::get(format!("http://127.0.0.1:8080/"))
            .send(&service)
            .await
            .take_string()
            .await
            .unwrap();

        assert_eq!(content, "Hello world");
    }
}