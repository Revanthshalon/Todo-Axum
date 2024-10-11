use crate::{error::TodoError, model::Todo};
use sqlx::PgPool;

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
        let todo = sqlx::query_as!(
            Todo,
            r#"
            INSERT INTO todos (title, description)
            VALUES ($1, $2)
            RETURNING *
            "#,
            title,
            description
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(todo)
    }

    pub async fn update_todo(
        &self,
        id: i32,
        title: Option<String>,
        description: Option<String>,
        completed: Option<bool>,
    ) -> Result<Todo, sqlx::Error> {
        let todo = sqlx::query_as!(
            Todo,
            r#"
            UPDATE todos
            SET title = COALESCE($2, title),
                description = COALESCE($3, description),
                completed = COALESCE($4, completed),
                updated_at = now()
            WHERE id = $1
            RETURNING *
            "#,
            id,
            title,
            description,
            completed
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(todo)
    }

    pub async fn get_todo(&self, id: i32) -> Result<Option<Todo>, TodoError> {
        let todo_option = sqlx::query_as!(
            Todo,
            r#"
            SELECT * FROM todos WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(todo_option)
    }

    pub async fn get_all_todos(&self) -> Result<Vec<Todo>, TodoError> {
        let todos = sqlx::query_as!(
            Todo,
            r#"
            SELECT * FROM todos
            "#,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(todos)
    }

    pub async fn delete_todo(&self, id: i32) -> Result<bool, TodoError> {
        let deleted_rows = sqlx::query!(
            r#"
            DELETE FROM todos WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        Ok(deleted_rows == 1)
    }
}
