use sqlx::PgPool;
use crate::{error::TodoError, model::Todo};

pub struct TodoRepository {
    pool: PgPool,
}

impl TodoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_todo(
        &self,
        title: String,
        description: Option<String>,
    ) -> Result<Todo, sqlx::Error> {
        todo!()
    }

    pub async fn update_todo(
        &self,
        id: i32,
        title: Option<String>,
        description: Option<String>,
        completed: Option<bool>,
    ) -> Result<Todo, sqlx::Error> {
        todo!()
    }

    pub async fn get_todo(&self, id: i32) -> Result<Option<Todo>, TodoError> {
        let record_option = sqlx::query!(
            r#"
            SELECT * FROM todos WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        let todo_option = record_option.map(|record| Todo {
            id: record.id,
            title: record.title,
            description: record.description,
            completed: record.completed,
            created_at: record.created_at.unwrap(),
            updated_at: record.updated_at,
        });
        Ok(todo_option)
    }

    pub async fn get_all_todos(&self) -> Result<Vec<Todo>, TodoError> {
        todo!()
    }

    pub async fn delete_todo(&self, id: i32) -> Result<bool, TodoError> {
        todo!()
    }
}
