use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub phone: Option<String>,
    pub activated: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
