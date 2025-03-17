use crate::modules::user::handler::{get_all, update_me, get_me, get_by_id};
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/me", web::get().to(get_me))
            .route("/me", web::put().to(update_me))
            .route("", web::get().to(get_all))
            .route("/{id}", web::get().to(get_by_id)),
    );
}
