use poem::{handler, http::StatusCode, IntoResponse, Response};

#[handler]
pub fn health_check() -> impl IntoResponse {
    Response::builder().status(StatusCode::OK).finish()
}
