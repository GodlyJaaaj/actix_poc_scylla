use crate::schema::reset_password_tokens;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, ToSchema, Debug)]
#[diesel(table_name = reset_password_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ResetPasswordToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
}
