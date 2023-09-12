use std::convert::From;

use chrono::{NaiveDateTime, TimeZone, Utc};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use uuid::Uuid;

use crate::schema;

#[derive(Clone, Copy, DbEnum, Debug, PartialEq, Eq, Hash)]
#[ExistingTypePath = "crate::schema::sql_types::TreeType"]
pub enum TreeType {
    Project,
    Folder,
    Inbox,
    Task,
    Document,
    Bug,
    Feature,
    Meeting,
}

impl From<teak_data::tree::TreeType> for TreeType {
    fn from(value: teak_data::tree::TreeType) -> Self {
        match value {
            teak_data::tree::TreeType::Project => TreeType::Project,
            teak_data::tree::TreeType::Folder => TreeType::Folder,
            teak_data::tree::TreeType::Inbox => TreeType::Inbox,
            teak_data::tree::TreeType::Task => TreeType::Task,
            teak_data::tree::TreeType::Document => TreeType::Document,
            teak_data::tree::TreeType::Bug => TreeType::Bug,
            teak_data::tree::TreeType::Feature => TreeType::Feature,
            teak_data::tree::TreeType::Meeting => TreeType::Meeting,
        }
    }
}

impl From<TreeType> for teak_data::tree::TreeType {
    fn from(value: TreeType) -> Self {
        match value {
            TreeType::Project => teak_data::tree::TreeType::Project,
            TreeType::Folder => teak_data::tree::TreeType::Folder,
            TreeType::Inbox => teak_data::tree::TreeType::Inbox,
            TreeType::Task => teak_data::tree::TreeType::Task,
            TreeType::Document => teak_data::tree::TreeType::Document,
            TreeType::Bug => teak_data::tree::TreeType::Bug,
            TreeType::Feature => teak_data::tree::TreeType::Feature,
            TreeType::Meeting => teak_data::tree::TreeType::Meeting,
        }
    }
}

#[derive(Clone, Copy, DbEnum, Debug, PartialEq, Eq, Hash)]
#[ExistingTypePath = "crate::schema::sql_types::TreeState"]
pub enum TreeState {
    Open,
    Closed,
}

impl From<teak_data::tree::TreeState> for TreeState {
    fn from(value: teak_data::tree::TreeState) -> Self {
        match value {
            teak_data::tree::TreeState::Open => TreeState::Open,
            teak_data::tree::TreeState::Closed => TreeState::Closed,
        }
    }
}

impl From<TreeState> for teak_data::tree::TreeState {
    fn from(value: TreeState) -> Self {
        match value {
            TreeState::Open => teak_data::tree::TreeState::Open,
            TreeState::Closed => teak_data::tree::TreeState::Closed,
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::tree)]
pub struct Tree {
    pub id: Uuid,
    pub path: Vec<Option<Uuid>>,
    pub title: String,
    pub type_: TreeType,
    pub state: TreeState,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl From<teak_data::tree::Tree> for Tree {
    fn from(value: teak_data::tree::Tree) -> Self {
        Tree {
            id: value.id,
            path: value.path.into_iter().map(|id| Some(id)).collect(),
            title: value.title,
            type_: value.typ.into(),
            state: value.state.into(),
            created_at: value.created_at.naive_utc(),
            updated_at: value.updated_at.naive_utc(),
            deleted_at: None,
        }
    }
}

impl From<Tree> for teak_data::tree::Tree {
    fn from(value: Tree) -> Self {
        teak_data::tree::Tree {
            id: value.id,
            path: value.path.into_iter().filter_map(|id| id).collect(),
            title: value.title,
            typ: value.type_.into(),
            state: value.state.into(),
            created_at: Utc.from_utc_datetime(&value.created_at),
            updated_at: Utc.from_utc_datetime(&value.updated_at),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = schema::tree)]
pub struct NewTree<'a> {
    pub id: &'a Uuid,
    pub path: &'a [Uuid],
    pub title: &'a str,
    pub type_: TreeType,
    pub state: TreeState,
}

impl<'a> From<&'a teak_data::tree::Tree> for NewTree<'a> {
    fn from(value: &'a teak_data::tree::Tree) -> Self {
        NewTree {
            id: &value.id,
            path: &value.path,
            title: &value.title,
            type_: value.typ.into(),
            state: value.state.into(),
        }
    }
}
