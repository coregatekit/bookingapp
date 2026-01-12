// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        #[max_length = 255]
        performer -> Varchar,
        date -> Timestamptz,
        #[max_length = 255]
        location -> Varchar,
        #[max_length = 50]
        status -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

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

diesel::table! {
    zones (id) {
        id -> Uuid,
        #[max_length = 50]
        label -> Varchar,
        price -> Numeric,
        total_seats -> Int4,
        event_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(zones -> events (event_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    users,
    zones,
);
