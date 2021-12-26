table! {
    password (id) {
        id -> Nullable<Integer>,
        key -> Text,
        value -> Text,
    }
}

table! {
    posts (id) {
        id -> Nullable<Integer>,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    password,
    posts,
);
