table! {
    categories (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
    }
}

table! {
    posts (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        category_id -> Unsigned<Integer>,
        title -> Varchar,
        content -> Text,
    }
}

table! {
    users (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        msg -> Nullable<Varchar>,
        age -> Nullable<Unsigned<Integer>>,
    }
}

joinable!(posts -> categories (category_id));
joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(categories, posts, users,);
