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

joinable!(posts_tags -> posts (post_id));
joinable!(posts_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(posts, posts_tags, tags,);
