use crate::config::Config;
use crate::modules::user::dto::UserUpdateQuery;
use crate::modules::user::service::UserService;
use crate::utils::response::{error, success};
use actix_files::NamedFile;
use actix_identity::Identity;
use actix_multipart::Multipart;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::io::ErrorKind;
use uuid::Uuid;
use validator::Validate;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn get_me(id: Identity, pool: web::Data<DbPool>) -> HttpResponse {
    // Get user ID from session
    let user_id = match id.id() {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::Unauthorized()
                .json(error(StatusCode::UNAUTHORIZED, "Not authenticated".into()));
        }
    };

    // Parse UUID
    let uuid = match Uuid::parse_str(&user_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(error(
                StatusCode::BAD_REQUEST,
                "Invalid user ID format".into(),
            ));
        }
    };

    // Get DB connection
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::ServiceUnavailable().json(error(
                StatusCode::SERVICE_UNAVAILABLE,
                "Database connection error".into(),
            ));
        }
    };

    // Get user
    match UserService::get_by_id(&mut conn, uuid) {
        Ok(user) => HttpResponse::Ok().json(success(StatusCode::OK, Some(user))),
        Err(e) => HttpResponse::NotFound().json(error(
            StatusCode::NOT_FOUND,
            format!("User not found: {}", e),
        )),
    }
}

pub async fn update_me(
    id: Identity,
    pool: web::Data<DbPool>,
    user_data: web::Json<UserUpdateQuery>,
) -> HttpResponse {
    // Validate update data
    if let Err(errors) = user_data.validate() {
        return HttpResponse::BadRequest().json(error(
            StatusCode::BAD_REQUEST,
            format!("Validation error: {:?}", errors),
        ));
    }

    // Get user ID from session
    let user_id = match id.id() {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::Unauthorized()
                .json(error(StatusCode::UNAUTHORIZED, "Not authenticated".into()));
        }
    };

    // Parse UUID
    let uuid = match Uuid::parse_str(&user_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(error(
                StatusCode::BAD_REQUEST,
                "Invalid user ID format".into(),
            ));
        }
    };

    // Get DB connection
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::ServiceUnavailable().json(error(
                StatusCode::SERVICE_UNAVAILABLE,
                "Database connection error".into(),
            ));
        }
    };

    // Update user
    match UserService::update_user(&mut conn, uuid, &user_data) {
        Ok(user) => HttpResponse::Ok().json(success(StatusCode::OK, Some(user))),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update user: {}", e),
        )),
    }
}

pub async fn update_avatar(
    id: Identity,
    pool: web::Data<DbPool>,
    mut payload: Multipart,
    config: web::Data<Config>,
) -> HttpResponse {
    // Get user ID from session
    let user_id = match id.id() {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::Unauthorized()
                .json(error(StatusCode::UNAUTHORIZED, "Not authenticated".into()));
        }
    };

    // Parse UUID
    let uuid = match Uuid::parse_str(&user_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(error(
                StatusCode::BAD_REQUEST,
                "Invalid user ID format".into(),
            ));
        }
    };

    // Get DB connection
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::ServiceUnavailable().json(error(
                StatusCode::SERVICE_UNAVAILABLE,
                "Database connection error".into(),
            ));
        }
    };

    // Update image
    match UserService::update_avatar(&mut conn, uuid, &mut payload, &config).await {
        Ok(user) => HttpResponse::Ok().json(success(StatusCode::OK, Some(user))),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update profile image: {}", e),
        )),
    }
}

pub async fn get_all(pool: web::Data<DbPool>) -> HttpResponse {
    // Get DB connection
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::ServiceUnavailable().json(error(
                StatusCode::SERVICE_UNAVAILABLE,
                "Database connection error".into(),
            ));
        }
    };

    // Get all users
    match UserService::get_all(&mut conn) {
        Ok(users) => HttpResponse::Ok().json(success(StatusCode::OK, Some(users))),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to retrieve users: {}", e),
        )),
    }
}

pub async fn get_by_id(path: web::Path<uuid::Uuid>, pool: web::Data<DbPool>) -> HttpResponse {
    let id = path.into_inner();

    // Get DB connection
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::ServiceUnavailable().json(error(
                StatusCode::SERVICE_UNAVAILABLE,
                "Database connection error".into(),
            ));
        }
    };

    // Get user
    match UserService::get_by_id(&mut conn, id) {
        Ok(user) => HttpResponse::Ok().json(success(StatusCode::OK, Some(user))),
        Err(e) => HttpResponse::NotFound().json(error(
            StatusCode::NOT_FOUND,
            format!("User not found: {}", e),
        )),
    }
}

pub async fn get_avatar(
    req: actix_web::HttpRequest,
    id: Identity,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
) -> HttpResponse {
    // Get user ID from session
    let user_id = match id.id() {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::Unauthorized()
                .json(error(StatusCode::UNAUTHORIZED, "Not authenticated".into()));
        }
    };

    // Parse UUID
    let uuid = match Uuid::parse_str(&user_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(error(
                StatusCode::BAD_REQUEST,
                "Invalid user ID format".into(),
            ));
        }
    };

    // Get DB connection
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::ServiceUnavailable().json(error(
                StatusCode::SERVICE_UNAVAILABLE,
                "Database connection error".into(),
            ));
        }
    };

    // Get image path from service
    match UserService::get_avatar_url(&mut conn, uuid, &config) {
        Ok(image_path) => match NamedFile::open(image_path) {
            Ok(file) => file.into_response(&req),
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    HttpResponse::NotFound().json(error(
                        StatusCode::NOT_FOUND,
                        "Profile image file not found".into(),
                    ))
                } else {
                    HttpResponse::InternalServerError().json(error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to retrieve profile image: {}", e),
                    ))
                }
            }
        },
        Err(e) => {
            let error_kind = if let Some(io_error) = e.downcast_ref::<std::io::Error>() {
                io_error.kind()
            } else {
                ErrorKind::Other
            };

            match error_kind {
                ErrorKind::NotFound => HttpResponse::NotFound().json(error(
                    StatusCode::NOT_FOUND,
                    "User has no profile image".into(),
                )),
                _ => HttpResponse::InternalServerError().json(error(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to retrieve profile image: {}", e),
                )),
            }
        }
    }
}
