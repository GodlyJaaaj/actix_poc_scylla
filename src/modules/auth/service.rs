use crate::modules::auth::dto::{RegisterQuery, LoginQuery};
use crate::models::User;
use crate::modules::auth::repository::AuthRepository;
use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest};
use diesel::prelude::*;
use diesel::PgConnection;
use std::error::Error;

pub struct AuthService;

impl AuthService {
    pub fn register(
        conn: &mut PgConnection,
        user_data: &RegisterQuery,
    ) -> Result<User, Box<dyn Error>> {
        // Check if user already exists
        if let Some(_) = AuthRepository::find_user_by_email(conn, &user_data.email)? {
            return Err("User with this email already exists".into());
        }

        // Create user and associated account
        let user = AuthRepository::create_user_account(conn, user_data)?;

        Ok(user)
    }

    pub fn login(
        req: &HttpRequest,
        conn: &mut PgConnection,
        login_data: &LoginQuery,
    ) -> Result<User, Box<dyn Error>> {
        use crate::models::Account;
        use crate::schema::accounts::dsl::*;
        use crate::utils::password::verify_password;

        // Find user by email
        let user = match AuthRepository::find_user_by_email(conn, &login_data.email)? {
            Some(user) => user,
            None => return Err("User not found".into()),
        };

        // Find the associated account with credentials
        let account = accounts
            .filter(user_id.eq(user.id))
            .filter(account_type.eq("credentials"))
            .filter(deleted_at.is_null())
            .first::<Account>(conn)
            .optional()?
            .ok_or("No credentials account found")?;

        // Get the stored password hash and verify
        let stored_hash = account.password.ok_or("No password set for this account")?;
        if !verify_password(&login_data.password, &stored_hash)? {
            return Err("Invalid password".into());
        }

        // Password verified, log the user in
        let _ = Identity::login(&req.extensions(), user.id.to_string());

        Ok(user)
    }

    pub fn logout(id: Identity) -> Result<(), Box<dyn Error>> {
        id.logout();
        Ok(())
    }
}
