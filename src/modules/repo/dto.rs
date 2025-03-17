use crate::schema::repositories;
use diesel::prelude::*;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Insertable, Deserialize, Validate, ToSchema)]
#[diesel(table_name = repositories)]
pub struct RepoCreateQuery {
    #[schema(example = "Acme Corporation")]
    #[validate(length(min = 3, max = 100))]
    pub name: String,

    #[schema(example = "https://git.acme.com/acme/acme-corporation.git")]
    pub url: String,

    #[schema(example = "git.acme.com")]
    pub host_name: Option<String>,

    #[schema(example = "git")]
    pub user_name: Option<String>,

    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub organization_id: Uuid,
}

#[derive(AsChangeset, Deserialize, Validate, ToSchema)]
#[diesel(table_name = repositories)]
pub struct RepoUpdateQuery {
    #[schema(example = "Acme Corporation Updated")]
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,

    #[schema(example = "https://git.acme.com/acme/acme-corporation.git")]
    pub url: Option<String>,
    
    #[schema(example = "git.acme.com")]
    pub host_name: Option<String>,
    
    #[schema(example = "git")]
    pub user_name: Option<String>,

    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub organization_id: Option<Uuid>,
}

