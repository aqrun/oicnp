table! {
    user_friends (id, friend_id) {
        id -> Int4,
        friend_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        en_name -> Varchar,
        user_type -> Varchar,
        appears_in -> Nullable<Varchar>,
        home_planet -> Nullable<Varchar>,
        primary_function -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    user_friends,
    users,
);
