use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use crate::schema::{users, accounts};

#[derive(Queryable, Selectable, Serialize, ToSchema, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub image: Option<String>,
    pub role: String,
    pub phone: Option<String>,
    pub verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable, Serialize, ToSchema, Debug)]
#[diesel(table_name = accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: Uuid,
    pub user_id: Uuid,
    pub account_type: String,
    pub password: Option<String>,
    pub provider: Option<String>,
    pub provider_account_id: Option<String>,
    pub refresh_token: Option<String>,
    pub access_token: Option<String>,
    pub expires_at: Option<NaiveDateTime>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

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

#[derive(Insertable, Deserialize, Validate, ToSchema)]
#[diesel(table_name = users)]
pub struct UpdateQuery {
    #[schema(example = "John Doe")]
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,

    #[schema(example = "+1234567890")]
    pub phone: Option<String>,

    #[schema(example = "https://example.com/image.jpg")]
    #[validate(url)]
    pub image: Option<String>,
}
