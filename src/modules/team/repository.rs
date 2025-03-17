use crate::models::Team;
use crate::modules::team::dto::{TeamCreateQuery, TeamUpdateQuery};
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use std::error::Error;
use uuid::Uuid;

use super::dto::AddUserToTeamQuery;

pub struct TeamRepository;

impl TeamRepository {
    pub fn find_by_id(conn: &mut PgConnection, team_id: Uuid) -> Result<Team, Box<dyn Error>> {
        use crate::schema::teams::dsl::*;

        let team = teams
            .filter(id.eq(team_id))
            .filter(deleted_at.is_null())
            .first::<Team>(conn)?;

        Ok(team)
    }

    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<Team>, Box<dyn Error>> {
        use crate::schema::teams::dsl::*;

        let all_teams = teams.filter(deleted_at.is_null()).load::<Team>(conn)?;

        Ok(all_teams)
    }

    pub fn create(
        conn: &mut PgConnection,
        new_team: &TeamCreateQuery,
    ) -> Result<Team, Box<dyn Error>> {
        use crate::schema::teams::dsl::*;

        diesel::insert_into(teams)
            .values(new_team)
            .get_result::<Team>(conn)
            .map_err(|e| e.into())
    }

    pub fn update(
        conn: &mut PgConnection,
        team_id: Uuid,
        update_data: &TeamUpdateQuery,
    ) -> Result<Team, Box<dyn Error>> {
        use crate::schema::teams::dsl::*;

        diesel::update(teams)
            .filter(id.eq(team_id))
            .filter(deleted_at.is_null())
            .set(update_data)
            .get_result::<Team>(conn)
            .map_err(|e| e.into())
    }

    pub fn delete(conn: &mut PgConnection, team_id: Uuid) -> Result<(), Box<dyn Error>> {
        use crate::schema::teams::dsl::*;

        diesel::update(teams)
            .filter(id.eq(team_id))
            .filter(deleted_at.is_null())
            .set(deleted_at.eq(Utc::now()))
            .execute(conn)?;

        Ok(())
    }

    pub fn add_user(
        conn: &mut PgConnection,
        team_id: Uuid,
        user_data: &AddUserToTeamQuery,
    ) -> Result<(), Box<dyn Error>> {
        use crate::schema::team_users;

        diesel::insert_into(team_users::table)
            .values((
                team_users::team_id.eq(team_id),
                team_users::user_id.eq(user_data.user_id),
                team_users::role.eq(user_data.role.clone()),
            ))
            .execute(conn)?;

        Ok(())
    }
}
