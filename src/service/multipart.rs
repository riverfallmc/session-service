#![allow(dead_code)]

use axum::{extract::Multipart, Json};
use adjust::response::{HttpError, HttpResult};
use reqwest::StatusCode;

pub struct MultipartService;

// Mb
const MAX_FILE_SIZE: usize = 2 * (1024 * 1024);

impl MultipartService {
  pub async fn read(mut multipart: Multipart) -> HttpResult<Vec<u8>> {
    let mut data = Vec::new();
    let mut total_size = 0;

    while let Some(mut field) = multipart.next_field().await.unwrap() {
      let content_type = field.content_type().unwrap_or("unknown");

      if content_type != "image/png" {
        return Err(HttpError::new("Можно установить только файлы формата .png!", Some(StatusCode::BAD_REQUEST)));
      }

      while let Some(chunk) = field.chunk().await.unwrap() {
        total_size += chunk.len();

        if total_size > MAX_FILE_SIZE {
          return Err(HttpError::new("Файл не должен быть больше 3 мегабайт!", Some(StatusCode::PAYLOAD_TOO_LARGE)));
        }

        data.extend_from_slice(&chunk);
      }
    }

    Ok(Json(data))
  }
}