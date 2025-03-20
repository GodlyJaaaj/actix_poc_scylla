use crate::models::Team;
use crate::modules::team::dto::{AddUserToTeamQuery, TeamCreateQuery, TeamUpdateQuery};
use crate::modules::team::repository::TeamRepository;
use diesel::PgConnection;
use std::error::Error;
use uuid::Uuid;

pub struct TeamService;

impl TeamService {
    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<Team>, Box<dyn Error>> {
        TeamRepository::find_all(conn)
    }

    pub fn get_by_id(conn: &mut PgConnection, team_id: Uuid) -> Result<Team, Box<dyn Error>> {
        TeamRepository::find_by_id(conn, team_id)
    }

    pub fn create(conn: &mut PgConnection, data: &TeamCreateQuery) -> Result<Team, Box<dyn Error>> {
        TeamRepository::create(conn, data)
    }

    pub fn update(
        conn: &mut PgConnection,
        team_id: Uuid,
        data: &TeamUpdateQuery,
    ) -> Result<Team, Box<dyn Error>> {
        TeamRepository::update(conn, team_id, data)
    }

    pub fn delete(conn: &mut PgConnection, team_id: Uuid) -> Result<(), Box<dyn Error>> {
        TeamRepository::delete(conn, team_id)
    }

    pub fn add_user(
        conn: &mut PgConnection,
        team_id: Uuid,
        user_data: &AddUserToTeamQuery,
    ) -> Result<(), Box<dyn Error>> {
        TeamRepository::add_user(conn, team_id, user_data)
    }
}
