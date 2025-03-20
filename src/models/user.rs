use crate::schema::users;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
