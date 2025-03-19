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
pub struct VerifyEmailParams {
    pub token: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct ForgotPasswordQuery {
    #[schema(example = "john.doe@gmail.com")]
    #[validate(email)]
    pub email: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct ResetPasswordQuery {
    #[schema(example = "reset_token_123456")]
    #[validate(length(min = 10))]
    pub token: String,

    #[schema(example = "new_password")]
    #[validate(length(min = 8, max = 100))]
    pub password: String,

    #[schema(example = "new_password")]
    #[validate(length(min = 8, max = 100))]
    pub password_confirmation: String,
}
