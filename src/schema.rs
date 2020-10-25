table! {
    attendance_records (id) {
        id -> Int4,
        user_id -> Int4,
        start_time -> Timestamp,
        end_time -> Timestamp,
        break_time -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        uid -> Varchar,
    }
}

joinable!(attendance_records -> users (user_id));

allow_tables_to_appear_in_same_query!(attendance_records, users,);
