pub struct AppConfig {
    todo_db_url: String,
}

impl AppConfig {
    pub fn load() -> Self {
        use dotenvy::dotenv;
        use std::env;

        dotenv().ok();

        let todo_db_url = env::var("TODO_DATABASE_URL").expect("DATABASE_URL must be set");

        Self { todo_db_url }
    }

    pub fn get_todo_db_url(&self) -> &str {
        &self.todo_db_url
    }
}
