use crate::modules::auth::handler::{login, logout, register, request_email_verification, verify_email, forgot_password, reset_password};
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
            .route("/request-email-verification", web::post().to(request_email_verification))
            .route("/verify-email", web::post().to(verify_email))
            .route("/forgot-password", web::post().to(forgot_password))
            .route("/reset-password", web::post().to(reset_password)),
    );
}
