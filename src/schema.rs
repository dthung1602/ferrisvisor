// @generated automatically by Diesel CLI.

diesel::table! {
    host (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        port -> Nullable<Integer>,
        username -> Nullable<Text>,
        password -> Nullable<Text>,
        created_at -> Nullable<Text>,
        updated_at -> Nullable<Text>,
    }
}

diesel::table! {
    permission (id) {
        id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        host_id -> Nullable<Integer>,
        service_name -> Nullable<Text>,
        can_view -> Nullable<Bool>,
        can_act -> Nullable<Bool>,
    }
}

diesel::table! {
    session (id) {
        id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        token -> Nullable<Text>,
        expires_at -> Nullable<Text>,
    }
}

diesel::table! {
    user (id) {
        id -> Nullable<Integer>,
        email -> Nullable<Text>,
        password -> Nullable<Text>,
        created_at -> Nullable<Text>,
        updated_at -> Nullable<Text>,
        last_login -> Nullable<Text>,
        is_admin -> Nullable<Bool>,
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
