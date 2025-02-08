use poem::{http::StatusCode, IntoResponse, Response};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub message: String,
    pub code: u16,
}

#[derive(Debug)]
pub enum JwtErrorKind {
    Expired,
    Invalid,
    Missing,
}

impl JwtErrorKind {
    pub fn message(&self) -> &'static str {
        match self {
            JwtErrorKind::Expired => "Token has expired",
            JwtErrorKind::Invalid => "Invalid token",
            JwtErrorKind::Missing => "Invalid authorization header",
        }
    }

    pub fn code(&self) -> u16 {
        match self {
            JwtErrorKind::Expired => StatusCode::BAD_REQUEST.as_u16(),
            JwtErrorKind::Invalid => StatusCode::BAD_REQUEST.as_u16(),
            JwtErrorKind::Missing => StatusCode::UNAUTHORIZED.as_u16(),
        }
    }
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

impl ApiError {
    pub fn middleware_response(kind: JwtErrorKind) -> Response {
        let message = kind.message();
        let code = kind.code();

        let body = serde_json::to_string(&json!({
            "message": message,
            "code": code,
        }))
        .unwrap_or_else(|_| {
            format!(
                r#"{{"message": "Internal server error", "code": {}}}"#,
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        });

        Response::builder()
            .status(StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
            .header("Content-type", "application/json")
            .body(body)
            .into_response()
    }
}
