use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TreeState {
    Open,
    Closed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tree {
    pub id: Uuid,
    pub path: Vec<Uuid>,
    pub title: String,
    pub typ: TreeType,
    pub state: TreeState,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
