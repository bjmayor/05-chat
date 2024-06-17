mod chat;
mod file;
mod message;
mod user;
mod workspace;
pub use chat::CreateChat;
pub use message::{CreateMessage, ListMessages};
use serde::{Deserialize, Serialize};
use sqlx::{
    types::chrono::{DateTime, Utc},
    FromRow,
};
pub use user::{CreateUser, SigninUser};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, Eq)]
pub struct User {
    pub id: i64,
    pub ws_id: i64,
    pub fullname: String,
    pub email: String,
    #[sqlx(default)]
    #[serde(skip)]
    pub password_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, Eq)]
pub struct ChatUser {
    pub id: i64,
    pub fullname: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, Eq)]
pub struct Workspace {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub owner_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "chat_type", rename_all = "snake_case")]
pub enum ChatType {
    Single,
    Group,
    PrivateChannel,
    PublicChannel,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, Eq)]
pub struct Chat {
    pub id: i64,
    pub ws_id: i64,
    pub name: Option<String>,
    pub r#type: ChatType,
    pub members: Vec<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatFile {
    pub ws_id: u64,  // workspace id
    pub ext: String, // extract ext from filename or mime type
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, Eq)]
pub(crate) struct Message {
    pub id: i64,
    pub chat_id: i64,
    pub sender_id: i64,
    pub content: String,
    pub files: Vec<String>,
    pub created_at: DateTime<Utc>,
}
