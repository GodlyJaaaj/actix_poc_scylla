use crate::modules::auth::routes as auth_routes;
use crate::modules::user::routes as user_routes;
use crate::modules::organization::routes as organization_routes;
use crate::modules::team::routes as team_routes;
use crate::modules::repo::routes as repo_routes;
use actix_web::{web, HttpResponse, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("/health").route(web::get().to(health_check)))
            .configure(auth_routes::config_routes)
            .configure(user_routes::config_routes)
            .configure(organization_routes::config_routes)
            .configure(team_routes::config_routes)
            .configure(repo_routes::config_routes),
    );
}
