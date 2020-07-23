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
