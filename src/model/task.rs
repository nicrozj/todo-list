use chrono::{NaiveTime, NaiveDate};
use anyhow::Result;

use crate::Database;

pub struct Task {
    id: u64,
    user_id: u64,
    title: String,
    start_time: NaiveTime,
    end_time: NaiveTime,
    day: NaiveDate,
    is_completed: i8
}

impl Task {
    pub async fn from_user_day(day: NaiveDate, user_id: i64, database: Database) -> Result<Vec<Self>> {
        let query = sqlx::query_as!(
            Task,
            "SELECT * FROM tasks WHERE user_id = ? AND day = ?",
            user_id,
            day,
        );

        let tasks = query.fetch_all(database.get_pool()).await?;

        Ok(tasks)
    }
}
