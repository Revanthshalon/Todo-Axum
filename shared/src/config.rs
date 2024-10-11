pub struct AppConfig {
    db_url: String,
}

impl AppConfig {
    pub fn load() -> Self {
        use dotenvy::dotenv;
        use std::env;

        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Self { db_url }
    }

    pub fn get_db_url(&self) -> &str {
        &self.db_url
    }
}
