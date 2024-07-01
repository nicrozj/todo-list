use sqlx::MySqlPool;

#[derive(Clone)]
pub struct Database(MySqlPool);

impl Database {
    pub fn from_env() -> Self {
        let addr = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL env var");

        let inner = MySqlPool::connect_lazy(&addr).expect("Error while connecting to database");

        Self(inner)
    }

    pub fn get_pool(&self) -> &MySqlPool {
        &self.0
    }
}
