use crate::modules::auth::routes as auth_routes;
use actix_web::{web, HttpResponse, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("/health").route(web::get().to(health_check)))
            .configure(auth_routes::config_routes),
    );
}
