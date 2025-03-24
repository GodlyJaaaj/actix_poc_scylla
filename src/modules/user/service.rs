use crate::config::Config;
use crate::models::User;
use crate::modules::user::dto::UserUpdateQuery;
use crate::modules::user::repository::UserRepository;
use crate::utils::image;
use actix_multipart::Multipart;
use actix_web::web;
use diesel::PgConnection;
use std::error::Error;
use std::io;
use std::path::PathBuf;
use uuid::Uuid;

pub struct UserService;

impl UserService {
    pub fn get_by_id(conn: &mut PgConnection, user_id: Uuid) -> Result<User, Box<dyn Error>> {
        UserRepository::find_by_id(conn, user_id)
    }

    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<User>, Box<dyn Error>> {
        UserRepository::find_all(conn)
    }

    pub fn update_user(
        conn: &mut PgConnection,
        user_id: Uuid,
        data: &UserUpdateQuery,
    ) -> Result<User, Box<dyn Error>> {
        UserRepository::update(conn, user_id, data)
    }

    pub async fn update_avatar(
        conn: &mut PgConnection,
        user_id: Uuid,
        payload: &mut Multipart,
        config: &web::Data<Config>,
    ) -> Result<User, Box<dyn Error>> {
        let image_path = match image::upload_image(payload, config, user_id).await {
            Ok(path) => path,
            Err(e) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to upload image: {}", e),
                )))
            }
        };

        UserRepository::update_avatar(conn, user_id, &image_path)
    }

    pub fn get_avatar_url(
        conn: &mut PgConnection,
        user_id: Uuid,
        config: &web::Data<Config>,
    ) -> Result<PathBuf, Box<dyn Error>> {
        let image_filename = match UserRepository::get_avatar(conn, user_id)? {
            Some(filename) if !filename.is_empty() => filename,
            _ => {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::NotFound,
                    "User has no profile image",
                )))
            }
        };

        Ok(image::get_image_path(config, user_id, &image_filename))
    }
}
