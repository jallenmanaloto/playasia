use crate::middleware::JwtMiddleware;
use crate::routes::{create, delete, edit, get_all_items, get_item, health_check};
use poem::{get, listener::TcpListener, EndpointExt, Route, Server};

#[tokio::main]
pub async fn run(address: String) -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/health", get(health_check))
        .at("/items", get(get_all_items).post(create))
        .at("/items/:id", get(get_item).put(edit).delete(delete))
        .with(JwtMiddleware::new("secret-key"));
    Server::new(TcpListener::bind(address)).run(app).await
}
