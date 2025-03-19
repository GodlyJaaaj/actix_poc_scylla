// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 50]
        account_type -> Varchar,
        #[max_length = 255]
        password -> Nullable<Varchar>,
        #[max_length = 50]
        provider -> Nullable<Varchar>,
        #[max_length = 255]
        provider_account_id -> Nullable<Varchar>,
        #[max_length = 255]
        refresh_token -> Nullable<Varchar>,
        #[max_length = 255]
        access_token -> Nullable<Varchar>,
        expires_at -> Nullable<Timestamptz>,
        #[max_length = 50]
        token_type -> Nullable<Varchar>,
        #[max_length = 255]
        scope -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    organization_users (organization_id, user_id) {
        organization_id -> Uuid,
        user_id -> Uuid,
        #[max_length = 50]
        role -> Nullable<Varchar>,
    }
}

diesel::table! {
    organizations (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    repositories (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 255]
        url -> Varchar,
        #[max_length = 255]
        host_name -> Nullable<Varchar>,
        #[max_length = 255]
        user_name -> Nullable<Varchar>,
        organization_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    team_users (team_id, user_id) {
        team_id -> Uuid,
        user_id -> Uuid,
        #[max_length = 50]
        role -> Nullable<Varchar>,
    }
}

diesel::table! {
    teams (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        organization_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        image -> Nullable<Varchar>,
        #[max_length = 50]
        role -> Varchar,
        #[max_length = 50]
        phone -> Nullable<Varchar>,
        verified -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    verification_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        token -> Varchar,
        created_at -> Timestamptz,
        expires_at -> Timestamptz,
        used_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(accounts -> users (user_id));
diesel::joinable!(organization_users -> organizations (organization_id));
diesel::joinable!(organization_users -> users (user_id));
diesel::joinable!(repositories -> organizations (organization_id));
diesel::joinable!(team_users -> teams (team_id));
diesel::joinable!(team_users -> users (user_id));
diesel::joinable!(teams -> organizations (organization_id));
diesel::joinable!(verification_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    organization_users,
    organizations,
    repositories,
    team_users,
    teams,
    users,
    verification_tokens,
);
