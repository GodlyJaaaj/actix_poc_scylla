use utoipa::OpenApi;

pub mod user;

#[derive(OpenApi)]
#[openapi(paths(
    crate::api::user::register,
    crate::api::user::login,
))]
pub struct UserApi;
