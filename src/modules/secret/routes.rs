use crate::modules::secret::handler::{
    create, delete, get_all_by_owner, get_by_id, get_value, update,
};
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/secrets")
            .route("", web::get().to(get_all_by_owner))
            .route("/{id}", web::get().to(get_by_id))
            .route("", web::post().to(create))
            .route("/{id}", web::put().to(update))
            .route("/{id}", web::delete().to(delete))
            .route("/value", web::post().to(get_value)),
    );
}
