use sqlx::PgPool;

use crate::AppError;

use super::Chat;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateChat {
    pub name: Option<String>,
    pub members: Vec<i64>,
}

#[allow(dead_code)]
impl Chat {
    pub async fn create(input: CreateChat, ws_id: u64, pool: &PgPool) -> Result<Self, AppError> {
        let chat = sqlx::query_as(
            r#"
				INSERT INTO chats (ws_id, name, members)
				VALUES ($1, $2, $3)
				RETURNING id, ws_id, name, r#type, members, created_at
				"#,
        )
        .bind(ws_id as i64)
        .bind(input.name)
        .bind(&input.members)
        .fetch_one(pool)
        .await?;
        Ok(chat)
    }
}
