use crate::models::User;
use crate::modules::user::repository::UserRepository;
use crate::modules::user::dto::UserUpdateQuery;
use diesel::PgConnection;
use std::error::Error;
use uuid::Uuid;

pub struct UserService;

impl UserService {
    pub fn get_by_id(conn: &mut PgConnection, user_id: Uuid) -> Result<User, Box<dyn Error>> {
        UserRepository::find_by_id(conn, user_id)
    }

    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<User>, Box<dyn Error>> {
        UserRepository::find_all(conn)
    }

    pub fn update_user(
        conn: &mut PgConnection,
        user_id: Uuid,
        data: &UserUpdateQuery,
    ) -> Result<User, Box<dyn Error>> {
        UserRepository::update(conn, user_id, data)
    }
}
