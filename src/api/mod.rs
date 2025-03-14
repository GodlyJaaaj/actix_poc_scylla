use utoipa::OpenApi;

pub mod pipeline;
pub mod user;

#[derive(OpenApi)]
#[openapi(paths(
    crate::api::user::register,
    crate::api::user::login,
    crate::api::user::logout,
    crate::api::user::me,
    crate::api::pipeline::get_pipeline_handler,
    crate::api::pipeline::update_pipeline_handler
))]
pub struct UserApi;
