use utoipa::OpenApi;

pub mod user;

#[derive(OpenApi)]
#[openapi(paths(
    crate::api::user::register,
    crate::api::user::login,
    crate::api::user::logout
))]
pub struct UserApi;
