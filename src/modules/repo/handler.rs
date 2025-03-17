use crate::modules::repo::dto::{RepoCreateQuery, RepoUpdateQuery};
use crate::modules::repo::service::RepoService;
use crate::utils::response::{error, success};
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use validator::Validate;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

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

    // Get all repos
    match RepoService::get_all(&mut conn) {
        Ok(repos) => HttpResponse::Ok().json(success(StatusCode::OK, Some(repos))),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to retrieve repos: {}", e),
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

    // Get repo by ID
    match RepoService::get_by_id(&mut conn, id) {
        Ok(repo) => HttpResponse::Ok().json(success(StatusCode::OK, Some(repo))),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to retrieve repo: {}", e),
        )),
    }
}

pub async fn create(
    pool: web::Data<DbPool>,
    repo_data: web::Json<RepoCreateQuery>,
) -> HttpResponse {
    // Validate repo data
    if let Err(errors) = repo_data.validate() {
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

    // Create repo
    match RepoService::create(&mut conn, &repo_data) {
        Ok(repo) => HttpResponse::Created().json(success(StatusCode::CREATED, Some(repo))),
        Err(e) => HttpResponse::BadRequest().json(error(
            StatusCode::BAD_REQUEST,
            format!("Failed to create repo: {}", e),
        )),
    }
}

pub async fn update(
    path: web::Path<uuid::Uuid>,
    pool: web::Data<DbPool>,
    repo_data: web::Json<RepoUpdateQuery>,
) -> HttpResponse {
    let id = path.into_inner();

    // Validate repo data
    if let Err(errors) = repo_data.validate() {
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

    // Update repo
    match RepoService::update(&mut conn, id, &repo_data) {
        Ok(repo) => HttpResponse::Ok().json(success(StatusCode::OK, Some(repo))),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update repo: {}", e),
        )),
    }
}

pub async fn delete(path: web::Path<uuid::Uuid>, pool: web::Data<DbPool>) -> HttpResponse {
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

    // Delete repo
    match RepoService::delete(&mut conn, id) {
        Ok(_) => HttpResponse::Ok().json(success::<()>(StatusCode::OK, None)),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete repo: {}", e),
        )),
    }
}
