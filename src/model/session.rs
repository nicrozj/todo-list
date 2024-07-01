use chrono::{NaiveDateTime, Utc};
use anyhow::Result;

use crate::Database;

#[derive(Debug)]
pub struct Session {
    id: u64,
    token: String,
    user_id: u64,
    user_agent: String,
    expire_date: NaiveDateTime
}

impl Session {
    pub fn get_user_id(&self) -> u64 {
        self.user_id
    }

    pub async fn from_token(token: &str, database: &Database) -> Result<Option<Self>> {
        let query = sqlx::query_as!(Session, "SELECT * FROM sessions WHERE token = ?", token);

        let session = query.fetch_optional(database.get_pool()).await?;

        Ok(session)
    }

    pub fn is_valid_token(&self) -> bool {
        Utc::now().naive_utc() < self.expire_date
    }

    pub fn is_valid_user_agent(&self, user_agent: &str) -> bool {
        self.user_agent == user_agent
    }
}
