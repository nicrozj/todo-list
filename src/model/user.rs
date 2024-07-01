use chrono::NaiveDateTime;
use anyhow::Result;
use sha2::{Sha256, Digest};

use crate::Database;

#[derive(Clone)]
pub struct User {
    id: u64,
    login: String,
    password: String,
    create_date: NaiveDateTime
}

impl User {
    pub fn get_login(&self) -> &str {
        &self.login
    }

    pub async fn from_id(id: u64, database: &Database) -> Result<Option<Self>> {
        let query = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id);

        let user = query.fetch_optional(database.get_pool()).await?;

        Ok(user)
    }

    pub async fn login(login: &str, password: &str, database: &Database) -> Result<bool> {
        let hash = password_hash(password, login);

        let query = sqlx::query!("SELECT id FROM users WHERE login = ? AND password = ?", login, hash);

        let is_valid = query.fetch_optional(database.get_pool()).await?.is_some();

        Ok(is_valid)
    }

    pub async fn signup(login: &str, password: &str, database: &Database) -> Result<u64> {
        let hash = password_hash(password, login);

        let query = sqlx::query!("INSERT INTO users(`login`, `password`) VALUES (?, ?)", login, hash);

        let id = query.execute(database.get_pool()).await?.last_insert_id();

        Ok(id)
    }

    pub async fn is_login_free(login: &str, database: &Database) -> Result<bool> {
        let query = sqlx::query!("SELECT id FROM users WHERE login = ?", login);

        let is_free = query.fetch_optional(database.get_pool()).await?.is_none();

        Ok(is_free)
    }

    pub async fn get_user_id_from_login(login: &str, database: &Database) -> Result<u64> {
        let query = sqlx::query!("SELECT id FROM users WHERE login = ?", login);
        let user_id = query.fetch_one(database.get_pool()).await?.id;
        Ok(user_id)
    }

    pub async fn create_token(user_id: u64, login: &str, user_agent: &str, expire_date: &NaiveDateTime, database: &Database) -> Result<String> {
        let mut func = Sha256::new();

        func.update(std::env::var("HASH_SALT").unwrap());
        func.update(user_id.to_be_bytes());
        func.update(std::env::var("HASH_SALT").unwrap());
        func.update(login);
        func.update(std::env::var("HASH_SALT").unwrap());
        func.update(expire_date.to_string());
        func.update(std::env::var("HASH_SALT").unwrap());
        func.update(user_agent);

        let token = format!("{:x}", func.finalize());

        let query = sqlx::query!(
            "INSERT INTO sessions(token, user_id, user_agent, expire_date) VALUES (?, ?, ?, ?)",
            token,
            user_id,
            user_agent,
            expire_date
        );

        query.execute(database.get_pool()).await?;

        Ok(token)
    }
}

fn password_hash(password: &str, login: &str) -> String{
    let mut func = Sha256::new();

    func.update(std::env::var("HASH_SALT").unwrap());
    func.update(password);
    func.update(std::env::var("HASH_SALT").unwrap());
    func.update(login);
    func.update(std::env::var("HASH_SALT").unwrap());

    format!("{:x}", func.finalize())
}
