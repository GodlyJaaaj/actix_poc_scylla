use crate::config::Config;
use crate::modules::auth::dto::{LoginQuery, RegisterQuery, ResetPasswordQuery, VerifyQuery};
use crate::modules::auth::service::AuthService;
use crate::utils::response::{error, success};
use actix_identity::Identity;
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use uuid::Uuid;
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

pub async fn request_verification(
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

    // Request email verification
    match AuthService::request_verification(&mut conn, uuid, &config) {
        Ok(_) => HttpResponse::Ok().json(success::<()>(StatusCode::OK, None)),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to send verification email: {}", e),
        )),
    }
}

pub async fn verify(pool: web::Data<DbPool>, token_data: web::Json<VerifyQuery>) -> HttpResponse {
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

    // Verify email
    match AuthService::verify(&mut conn, &token_data) {
        Ok(_) => HttpResponse::Ok().json(success::<()>(StatusCode::OK, None)),
        Err(e) => HttpResponse::BadRequest().json(error(
            StatusCode::BAD_REQUEST,
            format!("Email verification failed: {}", e),
        )),
    }
}

pub async fn forgot_password(
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

    // Send password reset email
    match AuthService::forgot_password(&mut conn, uuid, &config) {
        Ok(_) => HttpResponse::Ok().json(success::<()>(StatusCode::OK, None)),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to send verification email: {}", e),
        )),
    }
}

pub async fn reset_password(
    pool: web::Data<DbPool>,
    reset_data: web::Json<ResetPasswordQuery>,
) -> HttpResponse {
    // Validate reset data
    if let Err(errors) = reset_data.validate() {
        return HttpResponse::BadRequest().json(error(
            StatusCode::BAD_REQUEST,
            format!("Validation error: {:?}", errors),
        ));
    }

    // Check if passwords match
    if reset_data.password != reset_data.password_confirm {
        return HttpResponse::BadRequest().json(error(
            StatusCode::BAD_REQUEST,
            "Passwords do not match".into(),
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

    // Reset password
    match AuthService::reset_password(&mut conn, &reset_data) {
        Ok(_) => HttpResponse::Ok().json(success::<String>(
            StatusCode::OK,
            Some("Password has been reset successfully".into()),
        )),
        Err(e) => HttpResponse::BadRequest().json(error(
            StatusCode::BAD_REQUEST,
            format!("Password reset failed: {}", e),
        )),
    }
}
