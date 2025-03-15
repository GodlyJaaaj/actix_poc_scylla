use crate::models::Account;
use crate::models::User;
use crate::modules::auth::dto::RegisterQuery;
use diesel::prelude::*;
use diesel::PgConnection;
use std::error::Error;

pub struct AuthRepository;

impl AuthRepository {
    pub fn find_by_email(
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

    pub fn create(
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
            let account: Account = diesel::insert_into(accounts)
                .values((
                    user_id.eq(&user.id),
                    account_type.eq("credentials"),
                    password.eq(&hashed_password),
                ))
                .get_result(conn)?;

            log::info!("User created: {:?}", user);
            log::info!("Account created : {:?}", account);

            Ok(user)
        })
    }
}
