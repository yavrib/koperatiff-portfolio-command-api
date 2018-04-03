table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        published -> Bool,
        post_type -> Varchar,
        description -> Nullable<Varchar>,
        image_url -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
