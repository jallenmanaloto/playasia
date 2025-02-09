use crate::errors::ApiError;
use poem::web::{Json, Path};
use poem::{handler, http::StatusCode, Error as PoemError, IntoResponse};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs};

#[derive(Debug, Deserialize, Serialize, Clone)]
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

    fn write_to_file(file_path: Option<String>, data: Vec<Self>) -> Result<(), Box<dyn Error>> {
        let file_path = file_path.unwrap_or("data.json".to_string());
        let _ = fs::write(file_path, serde_json::to_string_pretty(&data)?);
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct RequestBody {
    pub name: String,
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

#[handler]
pub async fn create(Json(payload): Json<RequestBody>) -> Result<impl IntoResponse, PoemError> {
    let mut items = Item::get_all().map_err(|err| {
        PoemError::from_response(
            ApiError {
                message: format!("Failed to retrieve items: {}", err),
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            }
            .into_response(),
        )
    })?;

    let next_id = items.iter().map(|item| item.id).max().unwrap_or(0) + 1;

    let new_item = Item {
        id: next_id,
        name: payload.name,
    };

    items.push(new_item.clone());

    let _updated_data = serde_json::to_string_pretty(&items).map_err(|err| {
        PoemError::from_response(
            ApiError {
                message: format!("Failed to create an item: {}", err),
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            }
            .into_response(),
        )
    })?;

    let _ = Item::write_to_file(None, items).map_err(|err| {
        PoemError::from_response(
            ApiError {
                message: format!("Failed to write to file: {}", err),
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            }
            .into_response(),
        )
    })?;

    Ok(Json(new_item).with_status(StatusCode::CREATED))
}

#[handler]
pub fn edit(
    Path(id): Path<u16>,
    Json(payload): Json<RequestBody>,
) -> Result<impl IntoResponse, PoemError> {
    let mut items = Item::get_all().map_err(|err| {
        PoemError::from_response(
            ApiError {
                message: format!("Failed to retrieve items: {}", err),
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            }
            .into_response(),
        )
    })?;

    let item = items.iter_mut().find(|item| item.id == id).ok_or_else(|| {
        PoemError::from_response(
            ApiError {
                message: format!("Item with id {} not found", id),
                code: StatusCode::NOT_FOUND.as_u16(),
            }
            .into_response(),
        )
    })?;

    item.name = payload.name;
    let updated_item = item.clone();

    let _ = Item::write_to_file(None, items).map_err(|err| {
        PoemError::from_response(
            ApiError {
                message: format!("Failed to write to file: {}", err),
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            }
            .into_response(),
        )
    })?;

    Ok(Json(updated_item).with_status(StatusCode::OK))
}

#[derive(Serialize)]
pub struct DeletedMessageResponse {
    pub message: String,
}

#[handler]
pub fn delete(Path(id): Path<u16>) -> Result<impl IntoResponse, PoemError> {
    let mut items = Item::get_all().map_err(|err| {
        PoemError::from_response(
            ApiError {
                message: format!("Failed to retrieve items: {}", err),
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            }
            .into_response(),
        )
    })?;

    let item_position = items.iter().position(|item| item.id == id).ok_or_else(|| {
        PoemError::from_response(
            ApiError {
                message: format!("Item with id {} not found", id),
                code: StatusCode::NOT_FOUND.as_u16(),
            }
            .into_response(),
        )
    })?;

    let _ = items.remove(item_position);

    let _ = Item::write_to_file(None, items).map_err(|err| {
        PoemError::from_response(
            ApiError {
                message: format!("Failed to write to file: {}", err),
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            }
            .into_response(),
        )
    })?;

    Ok(Json(DeletedMessageResponse {
        message: "Item deleted successfully".to_string(),
    })
    .with_status(StatusCode::OK))
}
