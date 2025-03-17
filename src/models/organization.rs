use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::schema::organizations;

#[derive(Queryable, Selectable, Serialize, ToSchema, Debug)]
#[diesel(table_name = organizations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
