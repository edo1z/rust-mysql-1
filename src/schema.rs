table! {
    users (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        msg -> Nullable<Varchar>,
        age -> Nullable<Integer>,
    }
}
