use crate::models::User;
use crate::modules::user::dto::UserUpdateQuery;
use diesel::prelude::*;
use diesel::PgConnection;
use std::error::Error;
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    pub fn find_by_id(conn: &mut PgConnection, user_id: Uuid) -> Result<User, Box<dyn Error>> {
        use crate::schema::users::dsl::*;

        let user = users
            .filter(id.eq(user_id))
            .filter(deleted_at.is_null())
            .first::<User>(conn)?;

        Ok(user)
    }

    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<User>, Box<dyn Error>> {
        use crate::schema::users::dsl::*;

        let all_users = users.filter(deleted_at.is_null()).load::<User>(conn)?;

        Ok(all_users)
    }

    pub fn update(
        conn: &mut PgConnection,
        user_id: Uuid,
        update_data: &UserUpdateQuery,
    ) -> Result<User, Box<dyn Error>> {
        use crate::schema::users::dsl::*;

        diesel::update(users)
            .filter(id.eq(user_id))
            .filter(deleted_at.is_null())
            .set(update_data)
            .get_result::<User>(conn)
            .map_err(|e| e.into())
    }
}
