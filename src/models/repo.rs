use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::schema::repositories;

#[derive(Queryable, Selectable, Serialize, ToSchema, Debug)]
#[diesel(table_name = repositories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Repo {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub host_name: Option<String>,
    pub user_name: Option<String>,
    pub organization_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
