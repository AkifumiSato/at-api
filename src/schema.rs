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
    action_records_categories (record_id, category_id) {
        record_id -> Int4,
        category_id -> Int4,
    }
}

table! {
    categories (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Nullable<Varchar>,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        published_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    posts_tags (post_id, tag_id) {
        post_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
        slug -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
    }
}

joinable!(action_records -> users (user_id));
joinable!(action_records_categories -> action_records (record_id));
joinable!(action_records_categories -> categories (category_id));
joinable!(categories -> users (user_id));
joinable!(posts_tags -> posts (post_id));
joinable!(posts_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    action_records,
    action_records_categories,
    categories,
    posts,
    posts_tags,
    tags,
    users,
);
