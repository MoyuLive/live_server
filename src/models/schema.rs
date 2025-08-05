diesel::table! {
    users (id) {
        id -> BigInt,
        name -> Text,
        hashed_password -> Text,
    }
}

diesel::table! {
    rooms (id) {
        id -> BigInt,
        name -> Text,
        stream_key -> Text,
        user_id -> BigInt,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    stream_logs (id) {
        id -> BigInt,
        room_id -> BigInt,
        stream_type -> Text,
        action -> Text,
        client_id -> Text,
        ip -> Text,
        url -> Text,
        created_at -> Timestamp,
    }
}
