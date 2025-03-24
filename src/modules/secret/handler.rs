use crate::config::Config;
use crate::modules::secret::dto::{GetSecretValueQuery, SecretCreateQuery, SecretUpdateQuery};
use crate::modules::secret::service::SecretService;
use crate::utils::response::{error, success};
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use uuid::Uuid;
use validator::Validate;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn get_all_by_owner(
    query: web::Query<(Uuid, String)>,
    pool: web::Data<DbPool>,
) -> HttpResponse {
    let (owner_id, owner_type) = query.into_inner();

    // Validate owner type
    if !["organization", "team", "user"].contains(&owner_type.as_str()) {
        return HttpResponse::BadRequest().json(error(
            StatusCode::BAD_REQUEST,
            "Invalid owner type. Must be 'organization', 'team', or 'user'".into(),
        ));
    }

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

    // Get all secrets for this owner
    match SecretService::get_by_owner(&mut conn, owner_id, &owner_type) {
        Ok(secrets) => HttpResponse::Ok().json(success(StatusCode::OK, Some(secrets))),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to retrieve secrets: {}", e),
        )),
    }
}

pub async fn get_by_id(path: web::Path<Uuid>, pool: web::Data<DbPool>) -> HttpResponse {
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

    // Get secret by ID
    match SecretService::get_by_id(&mut conn, id) {
        Ok(secret) => HttpResponse::Ok().json(success(StatusCode::OK, Some(secret))),
        Err(e) => HttpResponse::NotFound().json(error(
            StatusCode::NOT_FOUND,
            format!("Secret not found: {}", e),
        )),
    }
}

pub async fn create(
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    secret_data: web::Json<SecretCreateQuery>,
) -> HttpResponse {
    // Validate secret data
    if let Err(errors) = secret_data.validate() {
        return HttpResponse::BadRequest().json(error(
            StatusCode::BAD_REQUEST,
            format!("Validation error: {:?}", errors),
        ));
    }

    // Validate owner type
    if !["organization", "team", "user"].contains(&secret_data.owner_type.as_str()) {
        return HttpResponse::BadRequest().json(error(
            StatusCode::BAD_REQUEST,
            "Invalid owner type. Must be 'organization', 'team', or 'user'".into(),
        ));
    }

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

    // Create secret
    match SecretService::create(&mut conn, &secret_data, &config.crypto.master_key) {
        Ok(secret) => HttpResponse::Created().json(success(StatusCode::CREATED, Some(secret))),
        Err(e) => HttpResponse::BadRequest().json(error(
            StatusCode::BAD_REQUEST,
            format!("Failed to create secret: {}", e),
        )),
    }
}

pub async fn update(
    path: web::Path<Uuid>,
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    secret_data: web::Json<SecretUpdateQuery>,
) -> HttpResponse {
    let id = path.into_inner();

    // Validate secret data
    if let Err(errors) = secret_data.validate() {
        return HttpResponse::BadRequest().json(error(
            StatusCode::BAD_REQUEST,
            format!("Validation error: {:?}", errors),
        ));
    }

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

    // Update secret
    match SecretService::update(&mut conn, id, &secret_data, &config.crypto.master_key) {
        Ok(secret) => HttpResponse::Ok().json(success(StatusCode::OK, Some(secret))),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update secret: {}", e),
        )),
    }
}

pub async fn delete(path: web::Path<Uuid>, pool: web::Data<DbPool>) -> HttpResponse {
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

    // Delete secret
    match SecretService::delete(&mut conn, id) {
        Ok(_) => HttpResponse::Ok().json(success::<()>(StatusCode::OK, None)),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete secret: {}", e),
        )),
    }
}

pub async fn get_value(
    pool: web::Data<DbPool>,
    config: web::Data<Config>,
    query: web::Json<GetSecretValueQuery>,
) -> HttpResponse {
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

    // Get and decrypt the secret value
    match SecretService::get_value(&mut conn, query.id, &config.crypto.master_key) {
        Ok(value) => HttpResponse::Ok().json(success(StatusCode::OK, Some(value))),
        Err(e) => HttpResponse::NotFound().json(error(
            StatusCode::NOT_FOUND,
            format!("Failed to retrieve secret value: {}", e),
        )),
    }
}
