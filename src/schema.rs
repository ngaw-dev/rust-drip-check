// @generated automatically by Diesel CLI.

diesel::table! {
    reminders (id) {
        id -> Integer,
        subscription_id -> Integer,
        days_before -> Integer,
        reminder_time -> Time,
    }
}

diesel::table! {
    subscriptions (id) {
        id -> Integer,
        title -> Text,
        price -> Integer,
        duration -> Text,
        start_date -> Date,
    }
}

diesel::joinable!(reminders -> subscriptions (subscription_id));

diesel::allow_tables_to_appear_in_same_query!(reminders, subscriptions,);
