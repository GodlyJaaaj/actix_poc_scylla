use crate::schema::secrets;
use diesel::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use std::sync::LazyLock;

static OWNER_TYPE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(organization|team|user)$").unwrap());

#[derive(Insertable, Deserialize, Validate, ToSchema)]
#[diesel(table_name = secrets)]
pub struct SecretCreateQuery {
    #[schema(example = "API Key")]
    #[validate(length(min = 3, max = 100))]
    pub name: String,

    #[schema(example = "API_KEY")]
    #[validate(length(min = 1, max = 255))]
    pub key: String,

    #[schema(example = "secret-value-here")]
    #[validate(length(min = 1))]
    pub value: String,

    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub owner_id: Uuid,

    #[schema(example = "organization")]
    #[validate(regex(path = *OWNER_TYPE_REGEX))]
    pub owner_type: String,
}

#[derive(AsChangeset, Deserialize, Validate, ToSchema)]
#[diesel(table_name = secrets)]
pub struct SecretUpdateQuery {
    #[schema(example = "Updated API Key")]
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,

    #[schema(example = "UPDATED_API_KEY")]
    #[validate(length(min = 1, max = 255))]
    pub key: Option<String>,

    #[schema(example = "updated-secret-value")]
    #[validate(length(min = 1))]
    pub value: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GetSecretValueQuery {
    pub id: Uuid,
}
