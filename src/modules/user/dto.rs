use crate::schema::users;
use diesel::prelude::*;
use regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;
use utoipa::ToSchema;
use validator::Validate;

static PHONE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\+[0-9]{5,15}$").unwrap());

#[derive(Insertable, Deserialize, Validate, ToSchema)]
#[diesel(table_name = users)]
pub struct UserUpdateQuery {
    #[schema(example = "John Doe")]
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,

    #[schema(example = "+1234567890")]
    #[validate(regex(path = *PHONE_REGEX))]
    pub phone: Option<String>,

    #[schema(example = "https://example.com/image.jpg")]
    #[validate(url)]
    pub image: Option<String>,
}
