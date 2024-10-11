use shared::config::AppConfig;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct DbService {
    pool: PgPool,
}

impl DbService {
    pub async fn new(app_config: &AppConfig) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&app_config.get_todo_db_url())
            .await
            .expect("Failed to create pool");
        Self { pool }
    }

    pub fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }
}
