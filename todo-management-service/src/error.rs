use thiserror::Error;

#[derive(Debug, Error)]
pub enum TodoError {
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),
}
