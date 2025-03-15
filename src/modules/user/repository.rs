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
        data: &UserUpdateQuery,
    ) -> Result<User, Box<dyn Error>> {
        use crate::schema::users::dsl::*;

        let target = users.filter(id.eq(user_id)).filter(deleted_at.is_null());

        if let Some(name_value) = &data.name {
            diesel::update(target.clone())
                .set(name.eq(name_value))
                .execute(conn)?;
        }

        if let Some(phone_value) = &data.phone {
            diesel::update(target.clone())
                .set(phone.eq(phone_value))
                .execute(conn)?;
        }

        if let Some(image_value) = &data.image {
            diesel::update(target.clone())
                .set(image.eq(image_value))
                .execute(conn)?;
        }

        // Get the updated user
        let updated_user = users
            .filter(id.eq(user_id))
            .filter(deleted_at.is_null())
            .first::<User>(conn)?;

        Ok(updated_user)
    }
}
