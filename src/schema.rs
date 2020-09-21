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
        category_id -> Nullable<Int4>,
    }
}

table! {
    posts (id) {
        id -> Int4,
        user_id -> Int4,
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
        user_id -> Int4,
        name -> Varchar,
        slug -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        uid -> Varchar,
    }
}

joinable!(action_categories -> users (user_id));
joinable!(action_records -> action_categories (category_id));
joinable!(action_records -> users (user_id));
joinable!(posts -> users (user_id));
joinable!(posts_tags -> posts (post_id));
joinable!(posts_tags -> tags (tag_id));
joinable!(tags -> users (user_id));

allow_tables_to_appear_in_same_query!(
    action_categories,
    action_records,
    posts,
    posts_tags,
    tags,
    users,
);
