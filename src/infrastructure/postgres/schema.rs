// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 15]
        mobile_phone -> Varchar,
        #[max_length = 10]
        gender -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
