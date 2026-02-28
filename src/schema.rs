// @generated automatically by Diesel CLI.

diesel::table! {
    group (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        created_at -> TimestamptzSqlite,
        updated_at -> TimestamptzSqlite,
    }
}

diesel::table! {
    host (id) {
        id -> Integer,
        group_id -> Integer,
        name -> Text,
        hostname -> Text,
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
        expires_at -> TimestamptzSqlite,
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

diesel::joinable!(host -> group (group_id));
diesel::joinable!(permission -> host (host_id));
diesel::joinable!(permission -> user (user_id));
diesel::joinable!(session -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(group, host, permission, session, user,);
