use utoipa::OpenApi;

pub mod user;

#[derive(OpenApi)]
#[openapi(paths(
    crate::api::user::register,
    crate::api::user::login,
    crate::api::user::logout,
    crate::api::user::me
))]
pub struct UserApi;
