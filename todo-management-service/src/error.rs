use thiserror::Error;

#[derive(Debug, Error)]
pub enum TodoError {
    #[error("Not found")]
    NotFound,
    #[error("Internal server error")]
    InternalServerError,
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),
}
