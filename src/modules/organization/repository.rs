use crate::models::Organization;
use crate::modules::organization::dto::{OrganizationCreateQuery, OrganizationUpdateQuery};
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use std::error::Error;
use uuid::Uuid;

use super::dto::AddUserToOrganizationQuery;

pub struct OrganizationRepository;

impl OrganizationRepository {
    pub fn find_by_id(
        conn: &mut PgConnection,
        organization_id: Uuid,
    ) -> Result<Organization, Box<dyn Error>> {
        use crate::schema::organizations::dsl::*;

        let organization = organizations
            .filter(id.eq(organization_id))
            .filter(deleted_at.is_null())
            .first::<Organization>(conn)?;

        Ok(organization)
    }

    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<Organization>, Box<dyn Error>> {
        use crate::schema::organizations::dsl::*;

        let all_organizations = organizations
            .filter(deleted_at.is_null())
            .load::<Organization>(conn)?;

        Ok(all_organizations)
    }

    pub fn create(
        conn: &mut PgConnection,
        new_organization: &OrganizationCreateQuery,
    ) -> Result<Organization, Box<dyn Error>> {
        use crate::schema::organizations::dsl::*;

        diesel::insert_into(organizations)
            .values(new_organization)
            .get_result::<Organization>(conn)
            .map_err(|e| e.into())
    }

    pub fn update(
        conn: &mut PgConnection,
        organization_id: Uuid,
        update_data: &OrganizationUpdateQuery,
    ) -> Result<Organization, Box<dyn Error>> {
        use crate::schema::organizations::dsl::*;

        diesel::update(organizations)
            .filter(id.eq(organization_id))
            .filter(deleted_at.is_null())
            .set(update_data)
            .get_result::<Organization>(conn)
            .map_err(|e| e.into())
    }

    pub fn delete(conn: &mut PgConnection, organization_id: Uuid) -> Result<(), Box<dyn Error>> {
        use crate::schema::organizations::dsl::*;

        diesel::update(organizations)
            .filter(id.eq(organization_id))
            .filter(deleted_at.is_null())
            .set(deleted_at.eq(Utc::now()))
            .execute(conn)?;

        Ok(())
    }

    pub fn add_user(
        conn: &mut PgConnection,
        organization_id: Uuid,
        user_data: &AddUserToOrganizationQuery,
    ) -> Result<(), Box<dyn Error>> {
        use crate::schema::organization_users;

        diesel::insert_into(organization_users::table)
            .values((
                organization_users::organization_id.eq(organization_id),
                organization_users::user_id.eq(user_data.user_id),
                organization_users::role.eq(user_data.role.clone()),
            ))
            .execute(conn)?;

        Ok(())
    }
}
