use std::path::{Path, PathBuf};
use axum::{body::Body, http::{header, StatusCode}, response::Response};
use dixxxie::response::HttpResult;
use tokio::{fs::{self, File}, io::AsyncWriteExt};

pub struct FileSystemService {}

impl FileSystemService {
  pub async fn read_file(path: PathBuf) -> HttpResult<Response> {
    let content = fs::read(path)
      .await?;

    Ok(Response::builder()
      .status(StatusCode::OK)
      .header(header::CONTENT_TYPE, "image/png")
      .body(Body::from(content))?)
  }

  pub async fn save(
    save_in: &str,
    filename: String,
    content: Vec<u8>
  ) -> HttpResult<()> {
    let path_str = format!("/app/data/{save_in}/{filename}");
    let path = Path::new(&path_str);

    if path.exists() {
      fs::remove_file(path)
        .await?;
    }

    let mut file = File::create_new(path)
      .await?;

    file.write_all(&content)
      .await?;

    Ok(())
  }
}