use crate::config::Config;
use actix_multipart::Multipart;
use actix_web::{web, Error};
use futures::StreamExt;
use sanitize_filename::sanitize;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use std::path::PathBuf;

pub async fn upload_image(
    payload: &mut Multipart,
    config: &web::Data<Config>,
    user_id: Uuid,
) -> Result<String, Error> {
    let mut filename = "".to_string();

    // Process each part of the multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item?;

        // Get content disposition
        let content_disposition = field.content_disposition().unwrap();

        // Get and validate content type
        let content_type = field
            .content_type()
            .map(|mime| mime.to_string())
            .unwrap_or_default();

        // Check if content type is allowed
        if !content_type.is_empty() && !config.file_storage.allowed_types.contains(&content_type) {
            return Err(Error::from(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unsupported file type: {}", content_type),
            )));
        }

        // Get the filename of the part
        let filename_str = content_disposition.get_filename().unwrap_or("unknown.file");
        filename = sanitize(filename_str);

        // Create a path to save the file
        let path = format!(
            "{}/{}",
            config.file_storage.upload_dir,
            format!("{}_{}", user_id, filename)
        );

        // Buffer for collecting file data
        let mut buffer = Vec::new();

        // Read all chunks into memory
        while let Some(chunk) = field.next().await {
            let data = chunk?;

            // Check running size
            buffer.extend_from_slice(&data);
            if buffer.len() > config.file_storage.max_size {
                // Clean up any existing file at the path
                if fs::metadata(&path).await.is_ok() {
                    let _ = fs::remove_file(&path).await;
                }

                return Err(Error::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "File size exceeded maximum allowed size of {} bytes",
                        config.file_storage.max_size
                    ),
                )));
            }
        }

        // Create a file
        let mut f = match fs::File::create(&path).await {
            Ok(file) => file,
            Err(e) => return Err(Error::from(e)),
        };

        // Write the complete file
        f.write_all(&buffer).await?;
    }

    Ok(filename)
}

pub fn get_image_path(config: &web::Data<Config>, user_id: Uuid, filename: &str) -> PathBuf {
    let sanitized_filename = sanitize(filename);
    let path = format!(
        "{}/{}",
        config.file_storage.upload_dir,
        format!("{}_{}", user_id, sanitized_filename)
    );
    PathBuf::from(path)
}
