use crate::schema::organizations;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Insertable, Deserialize, Validate, ToSchema)]
#[diesel(table_name = organizations)]
pub struct OrganizationCreateQuery {
    #[schema(example = "Acme Corporation")]
    #[validate(length(min = 3, max = 100))]
    pub name: String,

    #[schema(example = "A leading provider of innovative solutions")]
    pub description: Option<String>,
}

#[derive(AsChangeset, Deserialize, Validate, ToSchema)]
#[diesel(table_name = organizations)]
pub struct OrganizationUpdateQuery {
    #[schema(example = "Acme Corporation Updated")]
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,

    #[schema(example = "A leading provider of innovative solutions worldwide")]
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct AddUserToOrganizationQuery {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub user_id: Uuid,

    #[schema(example = "admin")]
    pub role: Option<String>,
}
