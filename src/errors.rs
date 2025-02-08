use poem::{http::StatusCode, IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub message: String,
    pub code: u16,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = serde_json::to_string(&self).unwrap_or_else(|_| {
            format!(
                r#"{{"message": "{}", "code": {}}}"#,
                self.message, self.code
            )
        });

        let status =
            StatusCode::from_u16(self.code).unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR);

        Response::builder()
            .status(status)
            .header("Content-type", "application/json")
            .body(body)
            .into_response()
    }
}
