use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
pub struct RegisterQuery {
    #[schema(example = "John Doe")]
    #[validate(length(min = 3, max = 100))]
    pub name: String,

    #[schema(example = "john.doe@gmail.com")]
    #[validate(email)]
    pub email: String,

    #[schema(example = "password")]
    #[validate(length(min = 8, max = 100))]
    pub password: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct LoginQuery {
    #[schema(example = "john.doe@gmail.com")]
    #[validate(email)]
    pub email: String,

    #[schema(example = "password")]
    #[validate(length(min = 8, max = 100))]
    pub password: String,
}

#[derive(Deserialize)]
pub struct VerifyQuery {
    pub token: String,
}

#[derive(Deserialize, Validate)]
pub struct ResetPasswordQuery {
    pub token: String,

    #[validate(length(min = 8, max = 100))]
    pub password: String,

    #[validate(length(min = 8, max = 100))]
    pub password_confirm: String,
}
