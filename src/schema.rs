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

diesel::joinable!(accounts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(accounts, users,);
