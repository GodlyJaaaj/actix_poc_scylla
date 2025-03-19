use crate::models::Account;
use crate::models::User;
use crate::models::VerificationToken;
use crate::modules::auth::dto::RegisterQuery;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;
use std::error::Error;
use uuid::Uuid;

pub struct AuthRepository;

impl AuthRepository {
    pub fn find_user_by_id(
        conn: &mut PgConnection,
        user_id: Uuid,
    ) -> Result<Option<User>, Box<dyn Error>> {
        use crate::schema::users::dsl::*;

        let user = users
            .filter(id.eq(user_id))
            .filter(deleted_at.is_null())
            .first::<User>(conn)
            .optional()?;

        Ok(user)
    }

    pub fn find_user_by_email(
        conn: &mut PgConnection,
        user_email: &str,
    ) -> Result<Option<User>, Box<dyn Error>> {
        use crate::schema::users::dsl::*;

        let user = users
            .filter(email.eq(user_email))
            .filter(deleted_at.is_null())
            .first::<User>(conn)
            .optional()?;

        Ok(user)
    }

    pub fn create_user_account(
        conn: &mut PgConnection,
        new_user: &RegisterQuery,
    ) -> Result<User, Box<dyn Error>> {
        use crate::schema::accounts::dsl::*;
        use crate::schema::users::dsl::*;

        conn.transaction(|conn| {
            // First create the user
            let user: User = diesel::insert_into(users)
                .values((name.eq(&new_user.name), email.eq(&new_user.email)))
                .get_result(conn)?;

            // Hash the password for account creation
            let hashed_password = crate::utils::password::hash_password(&new_user.password)?;

            // Create associated account with hashed password
            diesel::insert_into(accounts)
                .values((
                    user_id.eq(&user.id),
                    account_type.eq("credentials"),
                    password.eq(&hashed_password),
                ))
                .get_result::<Account>(conn)?;

            Ok(user)
        })
    }

    pub fn create_verification_token(
        conn: &mut PgConnection,
        new_user_id: Uuid,
        new_token: &str,
        new_expires_at: DateTime<Utc>,
    ) -> Result<(), Box<dyn Error>> {
        use crate::schema::verification_tokens::dsl::*;

        diesel::insert_into(verification_tokens)
            .values((
                user_id.eq(new_user_id),
                token.eq(new_token),
                expires_at.eq(new_expires_at),
            ))
            .execute(conn)?;

        Ok(())
    }

    pub fn find_verification_token(
        conn: &mut PgConnection,
        token_to_find: &str,
    ) -> Result<Option<VerificationToken>, Box<dyn Error>> {
        use crate::schema::verification_tokens::dsl::*;

        let verification_token = verification_tokens
            .filter(token.eq(token_to_find))
            .first::<VerificationToken>(conn)
            .optional()?;

        Ok(verification_token)
    }

    pub fn use_verification_token(
        conn: &mut PgConnection,
        token_to_use: &VerificationToken,
    ) -> Result<(), Box<dyn Error>> {
        use crate::schema::users;
        use crate::schema::verification_tokens;

        conn.transaction(|conn| {
            // Find the associated user
            let user = users::table
                .filter(users::id.eq(token_to_use.user_id))
                .filter(users::deleted_at.is_null())
                .first::<User>(conn)?;

            // Update the user's verified status
            diesel::update(users::table.filter(users::id.eq(user.id)))
                .set(users::verified.eq(true))
                .execute(conn)?;

            // Delete the verification token
            diesel::delete(
                verification_tokens::table.filter(verification_tokens::id.eq(token_to_use.id)),
            )
            .execute(conn)?;

            Ok(())
        })
    }
}
