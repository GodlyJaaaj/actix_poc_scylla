use crate::modules::user::handler::{get_all, update_me, update_avatar, get_me, get_by_id, get_avatar};
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/me", web::get().to(get_me))
            .route("/me", web::put().to(update_me))
            .route("/me/avatar", web::post().to(update_avatar))
            .route("/me/avatar", web::get().to(get_avatar))
            .route("", web::get().to(get_all))
            .route("/{id}", web::get().to(get_by_id)),
    );
}
