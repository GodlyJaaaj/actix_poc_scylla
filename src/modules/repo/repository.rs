use crate::models::Repo;
use crate::modules::repo::dto::{RepoCreateQuery, RepoUpdateQuery};
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use std::error::Error;
use uuid::Uuid;

pub struct RepoRepository;

impl RepoRepository {
    pub fn find_by_id(conn: &mut PgConnection, repo_id: Uuid) -> Result<Repo, Box<dyn Error>> {
        use crate::schema::repositories::dsl::*;

        let repo = repositories
            .filter(id.eq(repo_id))
            .filter(deleted_at.is_null())
            .first::<Repo>(conn)?;

        Ok(repo)
    }

    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<Repo>, Box<dyn Error>> {
        use crate::schema::repositories::dsl::*;

        let all_repos = repositories.filter(deleted_at.is_null()).load::<Repo>(conn)?;

        Ok(all_repos)
    }

    pub fn create(
        conn: &mut PgConnection,
        new_repo: &RepoCreateQuery,
    ) -> Result<Repo, Box<dyn Error>> {
        use crate::schema::repositories::dsl::*;

        diesel::insert_into(repositories)
            .values(new_repo)
            .get_result::<Repo>(conn)
            .map_err(|e| e.into())
    }

    pub fn update(
        conn: &mut PgConnection,
        repo_id: Uuid,
        update_data: &RepoUpdateQuery,
    ) -> Result<Repo, Box<dyn Error>> {
        use crate::schema::repositories::dsl::*;

        diesel::update(repositories)
            .filter(id.eq(repo_id))
            .filter(deleted_at.is_null())
            .set(update_data)
            .get_result::<Repo>(conn)
            .map_err(|e| e.into())
    }

    pub fn delete(conn: &mut PgConnection, repo_id: Uuid) -> Result<(), Box<dyn Error>> {
        use crate::schema::repositories::dsl::*;

        diesel::update(repositories)
            .filter(id.eq(repo_id))
            .filter(deleted_at.is_null())
            .set(deleted_at.eq(Utc::now()))
            .execute(conn)?;

        Ok(())
    }
}
