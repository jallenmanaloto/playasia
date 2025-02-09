use crate::errors::{ApiError, JwtErrorKind};
use jsonwebtoken::{decode, DecodingKey, Validation};
use poem::{
    error::Error as PoemError,
    http::{header, Method},
    Endpoint, Middleware, Request,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct JwtMiddleware {
    pub secret: String,
}

impl JwtMiddleware {
    pub fn new(secret: impl Into<String>) -> Self {
        Self {
            secret: secret.into(),
        }
    }
}

pub struct JwtMiddlewareImpl<E> {
    ep: E,
    secret: String,
}

impl<E: Endpoint> Middleware<E> for JwtMiddleware {
    type Output = JwtMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        JwtMiddlewareImpl {
            ep,
            secret: self.secret.clone(),
        }
    }
}

impl<E: Endpoint> Endpoint for JwtMiddlewareImpl<E> {
    type Output = E::Output;

    async fn call(&self, req: Request) -> poem::Result<Self::Output> {
        if req.method() == Method::GET {
            return self.ep.call(req).await;
        }

        let auth_header = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok());

        let token = match auth_header {
            Some(header) if header.starts_with("Bearer") => header[7..].to_string(),
            _ => {
                return Err(PoemError::from_response(ApiError::middleware_response(
                    JwtErrorKind::Missing,
                )))
            }
        };

        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(_token_data) => self.ep.call(req).await,
            Err(err) => match err.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => Err(PoemError::from_response(
                    ApiError::middleware_response(JwtErrorKind::Expired),
                )),
                _ => Err(PoemError::from_response(ApiError::middleware_response(
                    JwtErrorKind::Invalid,
                ))),
            },
        }
    }
}
