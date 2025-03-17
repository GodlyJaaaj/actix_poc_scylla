use crate::models::Organization;
use crate::modules::organization::dto::{
    AddUserToOrganizationQuery, OrganizationCreateQuery, OrganizationUpdateQuery,
};
use crate::modules::organization::repository::OrganizationRepository;
use diesel::PgConnection;
use std::error::Error;
use uuid::Uuid;

pub struct OrganizationService;

impl OrganizationService {
    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<Organization>, Box<dyn Error>> {
        OrganizationRepository::find_all(conn)
    }

    pub fn get_by_id(
        conn: &mut PgConnection,
        organization_id: Uuid,
    ) -> Result<Organization, Box<dyn Error>> {
        OrganizationRepository::find_by_id(conn, organization_id)
    }

    pub fn create(
        conn: &mut PgConnection,
        data: &OrganizationCreateQuery,
    ) -> Result<Organization, Box<dyn Error>> {
        OrganizationRepository::create(conn, data)
    }

    pub fn update(
        conn: &mut PgConnection,
        organization_id: Uuid,
        data: &OrganizationUpdateQuery,
    ) -> Result<Organization, Box<dyn Error>> {
        OrganizationRepository::update(conn, organization_id, data)
    }

    pub fn delete(conn: &mut PgConnection, organization_id: Uuid) -> Result<(), Box<dyn Error>> {
        OrganizationRepository::delete(conn, organization_id)
    }
    
    pub fn add_user(
        conn: &mut PgConnection,
        organization_id: Uuid,
        user_data: &AddUserToOrganizationQuery,
    ) -> Result<(), Box<dyn Error>> {
        OrganizationRepository::add_user(conn, organization_id, user_data)
    }
}
