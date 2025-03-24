use crate::models::SecretInfo;
use crate::modules::secret::dto::{SecretCreateQuery, SecretUpdateQuery};
use crate::modules::secret::repository::SecretRepository;
use diesel::PgConnection;
use std::error::Error;
use uuid::Uuid;

pub struct SecretService;

impl SecretService {
    pub fn get_by_id(
        conn: &mut PgConnection,
        secret_id: Uuid,
    ) -> Result<SecretInfo, Box<dyn Error>> {
        let secret = SecretRepository::find_by_id(conn, secret_id)?;
        Ok(secret.into())
    }

    pub fn get_by_owner(
        conn: &mut PgConnection,
        owner_id: Uuid,
        owner_type: &str,
    ) -> Result<Vec<SecretInfo>, Box<dyn Error>> {
        let secrets = SecretRepository::find_by_owner(conn, owner_id, owner_type)?;
        Ok(secrets.into_iter().map(|s| s.into()).collect())
    }

    pub fn create(
        conn: &mut PgConnection,
        data: &SecretCreateQuery,
        master_key: &str,
    ) -> Result<SecretInfo, Box<dyn Error>> {
        let secret = SecretRepository::create(conn, data, master_key)?;
        Ok(secret.into())
    }

    pub fn update(
        conn: &mut PgConnection,
        secret_id: Uuid,
        data: &SecretUpdateQuery,
        master_key: &str,
    ) -> Result<SecretInfo, Box<dyn Error>> {
        let secret = SecretRepository::update(conn, secret_id, data, master_key)?;
        Ok(secret.into())
    }

    pub fn delete(conn: &mut PgConnection, secret_id: Uuid) -> Result<(), Box<dyn Error>> {
        SecretRepository::delete(conn, secret_id)
    }

    pub fn get_value(
        conn: &mut PgConnection,
        secret_id: Uuid,
        master_key: &str,
    ) -> Result<String, Box<dyn Error>> {
        let secret = SecretRepository::find_by_id(conn, secret_id)?;
        SecretRepository::decrypt_value(&secret, master_key)
    }
}
