// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tree_state"))]
    pub struct TreeState;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tree_type"))]
    pub struct TreeType;
}

diesel::table! {
    graft (id) {
        id -> Uuid,
        node_path -> Array<Nullable<Uuid>>,
        target_path -> Array<Nullable<Uuid>>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TreeType;
    use super::sql_types::TreeState;

    tree (id) {
        id -> Uuid,
        path -> Array<Nullable<Uuid>>,
        title -> Text,
        #[sql_name = "type"]
        type_ -> TreeType,
        state -> TreeState,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    graft,
    tree,
);
