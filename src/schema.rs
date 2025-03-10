// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 10]
        role -> Varchar,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        activated -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
