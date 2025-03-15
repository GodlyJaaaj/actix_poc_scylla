use crate::modules::auth::models::{LoginQuery, RegisterQuery};
use crate::modules::auth::service::AuthService;
use crate::utils::response::{error, success};
use actix_identity::Identity;
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use validator::Validate;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn register(
    pool: web::Data<DbPool>,
    user_data: web::Json<RegisterQuery>,
) -> HttpResponse {
    // Validate user data
    if let Err(errors) = user_data.validate() {
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

    // Register user
    match AuthService::register(&mut conn, &user_data) {
        Ok(user) => HttpResponse::Created().json(success(StatusCode::CREATED, Some(user))),
        Err(e) => HttpResponse::BadRequest().json(error(
            StatusCode::BAD_REQUEST,
            format!("Registration failed: {}", e),
        )),
    }
}

pub async fn login(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    login_data: web::Json<LoginQuery>,
) -> HttpResponse {
    // Validate login data
    if let Err(errors) = login_data.validate() {
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

    // Login user
    match AuthService::login(&req, &mut conn, &login_data) {
        Ok(user) => HttpResponse::Ok().json(success(StatusCode::OK, Some(user))),
        Err(e) => HttpResponse::BadRequest().json(error(
            StatusCode::BAD_REQUEST,
            format!("Login failed: {}", e),
        )),
    }
}

pub async fn logout(id: Identity) -> HttpResponse {
    match AuthService::logout(id) {
        Ok(_) => HttpResponse::Ok().json(success::<()>(StatusCode::OK, None)),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Logout failed: {}", e),
        )),
    }
}
