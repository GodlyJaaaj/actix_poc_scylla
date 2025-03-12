use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,

    #[schema(example = "John Doe")]
    pub name: String,

    #[schema(example = "jhon.doe@gmail.com")]
    pub email: String,

    pub password: String,

    #[schema(example = "user")]
    pub role: String,

    pub phone: Option<String>,
    pub activated: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
