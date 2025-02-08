use crate::middleware::JwtMiddleware;
use crate::routes::{get_all_items, get_item, health_check};
use poem::{get, handler, listener::TcpListener, web::Path, EndpointExt, Route, Server};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[tokio::main]
pub async fn run(address: String) -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/health", get(health_check))
        .at("/items", get(get_all_items))
        .at("/items/:id", get(get_item))
        .at("/hello/:name", get(hello))
        .with(JwtMiddleware::new("secret-key"));
    Server::new(TcpListener::bind(address)).run(app).await
}
