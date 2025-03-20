use crate::models::Repo;
use crate::modules::repo::dto::{RepoCreateQuery, RepoUpdateQuery};
use crate::modules::repo::repository::RepoRepository;
use diesel::PgConnection;
use std::error::Error;
use uuid::Uuid;

pub struct RepoService;

impl RepoService {
    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<Repo>, Box<dyn Error>> {
        RepoRepository::find_all(conn)
    }

    pub fn get_by_id(conn: &mut PgConnection, repo_id: Uuid) -> Result<Repo, Box<dyn Error>> {
        RepoRepository::find_by_id(conn, repo_id)
    }

    pub fn create(conn: &mut PgConnection, data: &RepoCreateQuery) -> Result<Repo, Box<dyn Error>> {
        RepoRepository::create(conn, data)
    }

    pub fn update(
        conn: &mut PgConnection,
        repo_id: Uuid,
        data: &RepoUpdateQuery,
    ) -> Result<Repo, Box<dyn Error>> {
        RepoRepository::update(conn, repo_id, data)
    }

    pub fn delete(conn: &mut PgConnection, repo_id: Uuid) -> Result<(), Box<dyn Error>> {
        RepoRepository::delete(conn, repo_id)
    }
}
