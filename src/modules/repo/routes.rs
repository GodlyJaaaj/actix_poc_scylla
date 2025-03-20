use crate::modules::repo::handler::{create, delete, get_all, get_by_id, update};
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/repositories")
            .route("", web::get().to(get_all))
            .route("/{id}", web::get().to(get_by_id))
            .route("", web::post().to(create))
            .route("/{id}", web::put().to(update))
            .route("/{id}", web::delete().to(delete))
    );
}
