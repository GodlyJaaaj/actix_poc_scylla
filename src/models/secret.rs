use crate::schema::secrets;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, ToSchema, Debug)]
#[diesel(table_name = secrets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Secret {
    pub id: Uuid,
    pub name: String,
    pub key: String,
    #[serde(skip_serializing)]
    pub value: String, // Encrypted value, don't serialize directly
    #[serde(skip_serializing)]
    pub nonce: Vec<u8>, // Nonce for encryption
    pub owner_id: Uuid,
    pub owner_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

// Public representation of a secret (without the actual value)
#[derive(Serialize, ToSchema)]
pub struct SecretInfo {
    pub id: Uuid,
    pub name: String,
    pub key: String,
    pub owner_id: Uuid,
    pub owner_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Secret> for SecretInfo {
    fn from(secret: Secret) -> Self {
        SecretInfo {
            id: secret.id,
            name: secret.name,
            key: secret.key,
            owner_id: secret.owner_id,
            owner_type: secret.owner_type,
            created_at: secret.created_at,
            updated_at: secret.updated_at,
        }
    }
}
