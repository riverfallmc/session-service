use std::path::{Path, PathBuf};
use axum::{body::Body, http::{header, StatusCode}, response::Response};
use adjust::response::{HttpError, NonJsonHttpResult};
use tokio::{fs::{self, File}, io::AsyncWriteExt};

pub struct FileSystemService {}

impl FileSystemService {
  pub async fn read_file(path: PathBuf) -> NonJsonHttpResult<Response> {
    let content = fs::read(path)
      .await?;

    Ok(Response::builder()
      .status(StatusCode::OK)
      .header(header::CONTENT_TYPE, "image/png")
      .body(Body::from(content))?)
  }

  fn get_path(filename: &String) -> String {
    format!("/app/data/skincapes/{filename}")
  }

  pub async fn save(
    filename: String,
    content: Vec<u8>
  ) -> NonJsonHttpResult<()> {
    let path_str = Self::get_path(&filename);
    let path = Path::new(&path_str);

    if path.exists() {
      return Ok(())
    }

    let mut file = File::create_new(path)
      .await?;

    file.write_all(&content)
      .await?;

      Ok(())
  }

  pub async fn remove(filename: String) -> NonJsonHttpResult<()> {
    let path = Self::get_path(&filename);

    if !Path::new(&path).is_file() {
      return Err(HttpError::new("Файл не был найден", Some(StatusCode::NOT_FOUND)))
    }

    fs::remove_file(path)
      .await?;

    Ok(())
  }
}