table! {
    categories (id) {
        id -> Integer,
        name -> Varchar,
    }
}

table! {
    posts (id) {
        id -> Integer,
        user_id -> Integer,
        category_id -> Integer,
        title -> Varchar,
        content -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        msg -> Nullable<Varchar>,
        age -> Nullable<Integer>,
    }
}

joinable!(posts -> categories (category_id));
joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    categories,
    posts,
    users,
);
