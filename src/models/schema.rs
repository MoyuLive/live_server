diesel::table! {
    users (id) {
        id -> BigInt,
        name -> Text,
        hashed_password -> Text,
    }
}