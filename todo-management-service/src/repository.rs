use sqlx::PgPool;

use crate::{error::TodoError, model::Todo};

pub struct TodoRepository {
    pool: PgPool,
}

impl TodoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn create_todo(
        &self,
        title: String,
        description: Option<String>,
    ) -> Result<Todo, sqlx::Error> {
        todo!()
    }

    pub fn update_todo(
        &self,
        id: i32,
        title: Option<String>,
        description: Option<String>,
        completed: Option<bool>,
    ) -> Result<Todo, sqlx::Error> {
        todo!()
    }

    pub fn get_todo(&self, id: i32) -> Result<Option<Todo>, TodoError> {
        todo!()
    }

    pub fn get_all_todos(&self) -> Result<Vec<Todo>, TodoError> {
        todo!()
    }

    pub fn delete_todo(&self, id: i32) -> Result<bool, TodoError> {
        todo!()
    }
}
