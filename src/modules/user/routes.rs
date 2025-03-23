use crate::modules::user::handler::{get_all, update_me, update_profile_image, get_me, get_by_id, get_profile_image};
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/me", web::get().to(get_me))
            .route("/me", web::put().to(update_me))
            .route("/me/profile-image", web::post().to(update_profile_image))
            .route("/me/profile-image", web::get().to(get_profile_image))
            .route("", web::get().to(get_all))
            .route("/{id}", web::get().to(get_by_id)),
    );
}
