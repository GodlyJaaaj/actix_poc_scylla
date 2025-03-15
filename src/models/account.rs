use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::schema::accounts;

#[derive(Queryable, Selectable, Serialize, ToSchema, Debug)]
#[diesel(table_name = accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: Uuid,
    pub user_id: Uuid,
    pub account_type: String,
    pub password: Option<String>,
    pub provider: Option<String>,
    pub provider_account_id: Option<String>,
    pub refresh_token: Option<String>,
    pub access_token: Option<String>,
    pub expires_at: Option<NaiveDateTime>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
