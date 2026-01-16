// @generated automatically by Diesel CLI.

diesel::table! {
    booking_seats (id) {
        id -> Uuid,
        booking_id -> Uuid,
        seat_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    bookings (id) {
        id -> Uuid,
        event_id -> Uuid,
        total_price -> Numeric,
        #[max_length = 50]
        status -> Varchar,
        user_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

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
    seats (id) {
        id -> Uuid,
        #[max_length = 10]
        seat_row -> Varchar,
        #[max_length = 10]
        seat_column -> Varchar,
        #[max_length = 10]
        seat_number -> Varchar,
        is_reserved -> Bool,
        zone_id -> Uuid,
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

diesel::joinable!(booking_seats -> bookings (booking_id));
diesel::joinable!(booking_seats -> seats (seat_id));
diesel::joinable!(bookings -> users (user_id));
diesel::joinable!(seats -> zones (zone_id));
diesel::joinable!(zones -> events (event_id));

diesel::allow_tables_to_appear_in_same_query!(
    booking_seats,
    bookings,
    events,
    seats,
    users,
    zones,
);
