table! {
    action_categories (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
    }
}

table! {
    action_records (id) {
        id -> Int4,
        user_id -> Int4,
        start_time -> Timestamp,
        end_time -> Timestamp,
        info -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Int4,
        uid -> Varchar,
    }
}

joinable!(action_categories -> users (user_id));
joinable!(action_records -> users (user_id));

allow_tables_to_appear_in_same_query!(
    action_categories,
    action_records,
    users,
);
