use crate::modules::team::dto::{AddUserToTeamQuery, TeamCreateQuery, TeamUpdateQuery};
use crate::modules::team::service::TeamService;
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

    // Get all teams
    match TeamService::get_all(&mut conn) {
        Ok(teams) => HttpResponse::Ok().json(success(StatusCode::OK, Some(teams))),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to retrieve teams: {}", e),
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

    // Get team by ID
    match TeamService::get_by_id(&mut conn, id) {
        Ok(team) => HttpResponse::Ok().json(success(StatusCode::OK, Some(team))),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to retrieve team: {}", e),
        )),
    }
}

pub async fn create(
    pool: web::Data<DbPool>,
    team_data: web::Json<TeamCreateQuery>,
) -> HttpResponse {
    // Validate team data
    if let Err(errors) = team_data.validate() {
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

    // Create team
    match TeamService::create(&mut conn, &team_data) {
        Ok(team) => HttpResponse::Created().json(success(StatusCode::CREATED, Some(team))),
        Err(e) => HttpResponse::BadRequest().json(error(
            StatusCode::BAD_REQUEST,
            format!("Failed to create team: {}", e),
        )),
    }
}

pub async fn update(
    path: web::Path<uuid::Uuid>,
    pool: web::Data<DbPool>,
    team_data: web::Json<TeamUpdateQuery>,
) -> HttpResponse {
    let id = path.into_inner();

    // Validate team data
    if let Err(errors) = team_data.validate() {
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

    // Update team
    match TeamService::update(&mut conn, id, &team_data) {
        Ok(team) => HttpResponse::Ok().json(success(StatusCode::OK, Some(team))),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update team: {}", e),
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

    // Delete team
    match TeamService::delete(&mut conn, id) {
        Ok(_) => HttpResponse::Ok().json(success::<()>(StatusCode::OK, None)),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete team: {}", e),
        )),
    }
}

pub async fn add_user(
    path: web::Path<uuid::Uuid>,
    pool: web::Data<DbPool>,
    user_data: web::Json<AddUserToTeamQuery>,
) -> HttpResponse {
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

    // Add user to team
    match TeamService::add_user(&mut conn, id, &user_data) {
        Ok(_) => HttpResponse::Ok().json(success::<()>(StatusCode::OK, None)),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to add user to team: {}", e),
        )),
    }
}
