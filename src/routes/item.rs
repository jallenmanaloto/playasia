use crate::errors::ApiError;
use poem::web::{Json, Path};
use poem::{handler, http::StatusCode, Error as PoemError, IntoResponse};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs};

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub id: u16,
    pub name: String,
}

impl Item {
    fn get_all() -> Result<Vec<Self>, Box<dyn Error>> {
        let data = fs::read_to_string("data.json")?;
        if data.trim().is_empty() {
            return Ok(Vec::new());
        }

        let items = serde_json::from_str(&data)?;
        Ok(items)
    }
}

#[handler]
pub fn get_all_items() -> Result<impl IntoResponse, PoemError> {
    let items = Item::get_all().map_err(|err| {
        PoemError::from_response(
            ApiError {
                message: format!("Failed to retrieve items: {}", err),
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            }
            .into_response(),
        )
    })?;

    Ok(Json(items))
}

#[handler]
pub fn get_item(Path(id): Path<u16>) -> Result<impl IntoResponse, PoemError> {
    let items = Item::get_all().map_err(|err| {
        PoemError::from_response(
            ApiError {
                message: format!("Failed to retrieve items: {}", err),
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            }
            .into_response(),
        )
    })?;

    match items.into_iter().find(|item| item.id == id) {
        Some(item) => Ok(Json(item)),
        None => Err(PoemError::from_response(
            ApiError {
                message: "Item not found".to_string(),
                code: StatusCode::NOT_FOUND.as_u16(),
            }
            .into_response(),
        )),
    }
}
