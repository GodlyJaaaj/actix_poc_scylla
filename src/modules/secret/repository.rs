use crate::models::Secret;
use crate::modules::secret::dto::{SecretCreateQuery, SecretUpdateQuery};
use crate::utils::encryption;
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use std::error::Error;
use uuid::Uuid;

pub struct SecretRepository;

impl SecretRepository {
    pub fn find_by_id(conn: &mut PgConnection, secret_id: Uuid) -> Result<Secret, Box<dyn Error>> {
        use crate::schema::secrets::dsl::*;

        let secret = secrets
            .filter(id.eq(secret_id))
            .filter(deleted_at.is_null())
            .first::<Secret>(conn)?;

        Ok(secret)
    }

    pub fn find_by_owner(
        conn: &mut PgConnection,
        owner_id_param: Uuid,
        owner_type_param: &str,
    ) -> Result<Vec<Secret>, Box<dyn Error>> {
        use crate::schema::secrets::dsl::*;

        let results = secrets
            .filter(owner_id.eq(owner_id_param))
            .filter(owner_type.eq(owner_type_param))
            .filter(deleted_at.is_null())
            .load::<Secret>(conn)?;

        Ok(results)
    }

    pub fn find_by_key(
        conn: &mut PgConnection,
        key_param: &str,
        owner_id_param: Uuid,
        owner_type_param: &str,
    ) -> Result<Option<Secret>, Box<dyn Error>> {
        use crate::schema::secrets::dsl::*;

        let secret = secrets
            .filter(key.eq(key_param))
            .filter(owner_id.eq(owner_id_param))
            .filter(owner_type.eq(owner_type_param))
            .filter(deleted_at.is_null())
            .first::<Secret>(conn)
            .optional()?;

        Ok(secret)
    }

    pub fn create(
        conn: &mut PgConnection,
        new_secret: &SecretCreateQuery,
        master_key: &str,
    ) -> Result<Secret, Box<dyn Error>> {
        use crate::schema::secrets::dsl::*;

        // Check if a secret with the same key already exists for this owner
        let existing = Self::find_by_key(
            conn,
            &new_secret.key,
            new_secret.owner_id,
            &new_secret.owner_type,
        )?;

        if existing.is_some() {
            return Err(format!(
                "A secret with key '{}' already exists for this owner",
                new_secret.key
            )
            .into());
        }

        // Encrypt the value
        let (encrypted_value, nonce_bytes) =
            encryption::encrypt_value(&new_secret.value, master_key)?;

        // Insert with encrypted value
        let result = diesel::insert_into(secrets)
            .values((
                name.eq(&new_secret.name),
                key.eq(&new_secret.key),
                value.eq(encrypted_value),
                nonce.eq(nonce_bytes),
                owner_id.eq(new_secret.owner_id),
                owner_type.eq(&new_secret.owner_type),
            ))
            .get_result::<Secret>(conn)?;

        Ok(result)
    }

    pub fn update(
        conn: &mut PgConnection,
        secret_id: Uuid,
        update_data: &SecretUpdateQuery,
        master_key: &str,
    ) -> Result<Secret, Box<dyn Error>> {
        use crate::schema::secrets::dsl::*;

        // First, get the existing secret
        let existing = Self::find_by_id(conn, secret_id)?;

        // Start a transaction
        conn.transaction(|conn| {
            // Handle updating the name if provided
            if let Some(new_name) = &update_data.name {
                diesel::update(secrets)
                    .filter(id.eq(secret_id))
                    .filter(deleted_at.is_null())
                    .set(name.eq(new_name))
                    .execute(conn)?;
            }

            // Handle updating the key if provided
            if let Some(new_key) = &update_data.key {
                // Check if the new key already exists for this owner
                if new_key != &existing.key {
                    let key_exists = secrets
                        .filter(key.eq(new_key))
                        .filter(owner_id.eq(existing.owner_id))
                        .filter(owner_type.eq(&existing.owner_type))
                        .filter(deleted_at.is_null())
                        .filter(id.ne(secret_id)) // Exclude the current secret
                        .first::<Secret>(conn)
                        .optional()?
                        .is_some();

                    if key_exists {
                        return Err(diesel::result::Error::RollbackTransaction);
                    }
                }

                diesel::update(secrets)
                    .filter(id.eq(secret_id))
                    .filter(deleted_at.is_null())
                    .set(key.eq(new_key))
                    .execute(conn)?;
            }

            // Handle updating the value if provided
            if let Some(new_value) = &update_data.value {
                let (encrypted_value, nonce_bytes) =
                    encryption::encrypt_value(new_value, master_key)
                        .map_err(|_| diesel::result::Error::RollbackTransaction)?;

                diesel::update(secrets)
                    .filter(id.eq(secret_id))
                    .filter(deleted_at.is_null())
                    .set((value.eq(encrypted_value), nonce.eq(nonce_bytes)))
                    .execute(conn)?;
            }

            // Get the updated secret
            Ok(secrets
                .filter(id.eq(secret_id))
                .filter(deleted_at.is_null())
                .first::<Secret>(conn)?)
        })
        .map_err(|e| {
            if let diesel::result::Error::RollbackTransaction = e {
                format!("A secret with the same key already exists for this owner").into()
            } else {
                e.into()
            }
        })
    }

    pub fn delete(conn: &mut PgConnection, secret_id: Uuid) -> Result<(), Box<dyn Error>> {
        use crate::schema::secrets::dsl::*;

        diesel::update(secrets)
            .filter(id.eq(secret_id))
            .filter(deleted_at.is_null())
            .set(deleted_at.eq(Utc::now()))
            .execute(conn)?;

        Ok(())
    }

    pub fn decrypt_value(secret: &Secret, master_key: &str) -> Result<String, Box<dyn Error>> {
        encryption::decrypt_value(&secret.value, &secret.nonce, master_key)
    }
}
