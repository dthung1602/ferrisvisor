// @generated automatically by Diesel CLI.

diesel::table! {
    host (id) {
        id -> Integer,
        name -> Text,
        port -> Integer,
        username -> Nullable<Text>,
        password -> Nullable<Text>,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
    }
}

diesel::table! {
    permission (id) {
        id -> Integer,
        user_id -> Integer,
        host_id -> Integer,
        service_name -> Text,
        can_view -> Bool,
        can_act -> Bool,
    }
}

diesel::table! {
    session (id) {
        id -> Integer,
        user_id -> Integer,
        token -> Text,
        expires_at -> Text,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        email -> Text,
        password -> Text,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
        last_login -> Nullable<TimestamptzSqlite>,
        is_admin -> Bool,
    }
}

diesel::joinable!(permission -> host (host_id));
diesel::joinable!(permission -> user (user_id));
diesel::joinable!(session -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    host,
    permission,
    session,
    user,
);
