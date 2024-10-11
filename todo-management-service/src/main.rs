use repository::TodoRepository;
use service::{todo::todo_service_server, TodoService};
use shared::config::AppConfig;
use shared::db::DbService;
use tonic::transport::Server;

mod error;
mod model;
mod repository;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::load();
    let db_service = DbService::new(&app_config).await;
    let repository = TodoRepository::new(db_service.get_pool());
    let service = TodoService::new(repository);

    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(todo_service_server::TodoServiceServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
