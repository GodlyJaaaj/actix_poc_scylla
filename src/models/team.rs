use crate::schema::teams;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, ToSchema, Debug)]
#[diesel(table_name = teams)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub organization_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
