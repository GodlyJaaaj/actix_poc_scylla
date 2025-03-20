use crate::modules::organization::dto::{
    AddUserToOrganizationQuery, OrganizationCreateQuery, OrganizationUpdateQuery,
};
use crate::modules::organization::service::OrganizationService;
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

    // Get all organizations
    match OrganizationService::get_all(&mut conn) {
        Ok(organizations) => HttpResponse::Ok().json(success(StatusCode::OK, Some(organizations))),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to retrieve organizations: {}", e),
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

    // Get organization by ID
    match OrganizationService::get_by_id(&mut conn, id) {
        Ok(organization) => HttpResponse::Ok().json(success(StatusCode::OK, Some(organization))),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to retrieve organization: {}", e),
        )),
    }
}

pub async fn create(
    pool: web::Data<DbPool>,
    organization_data: web::Json<OrganizationCreateQuery>,
) -> HttpResponse {
    // Validate organization data
    if let Err(errors) = organization_data.validate() {
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

    // Create organization
    match OrganizationService::create(&mut conn, &organization_data) {
        Ok(organization) => {
            HttpResponse::Created().json(success(StatusCode::CREATED, Some(organization)))
        }
        Err(e) => HttpResponse::BadRequest().json(error(
            StatusCode::BAD_REQUEST,
            format!("Failed to create organization: {}", e),
        )),
    }
}

pub async fn update(
    path: web::Path<uuid::Uuid>,
    pool: web::Data<DbPool>,
    organization_data: web::Json<OrganizationUpdateQuery>,
) -> HttpResponse {
    let id = path.into_inner();

    // Validate organization data
    if let Err(errors) = organization_data.validate() {
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

    // Update organization
    match OrganizationService::update(&mut conn, id, &organization_data) {
        Ok(organization) => HttpResponse::Ok().json(success(StatusCode::OK, Some(organization))),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update organization: {}", e),
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

    // Delete organization
    match OrganizationService::delete(&mut conn, id) {
        Ok(_) => HttpResponse::Ok().json(success::<()>(StatusCode::OK, None)),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete organization: {}", e),
        )),
    }
}

pub async fn add_user(
    path: web::Path<uuid::Uuid>,
    pool: web::Data<DbPool>,
    user_data: web::Json<AddUserToOrganizationQuery>,
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

    // Add user to organization
    match OrganizationService::add_user(&mut conn, id, &user_data) {
        Ok(_) => HttpResponse::Ok().json(success::<()>(StatusCode::OK, None)),
        Err(e) => HttpResponse::InternalServerError().json(error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to add user to organization: {}", e),
        )),
    }
}
