// @generated automatically by Diesel CLI.

diesel::table! {
    subscriptions (id) {
        id -> Integer,
        title -> Text,
        price -> Integer,
        duration -> Text,
        start_date -> Date,
    }
}
