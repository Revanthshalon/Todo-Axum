use crate::clients::todo_client::TodoClient;
use crate::schema::AppSchema;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::routing::post;
use axum::{Extension, Router};
use tokio::net::TcpListener;

mod clients;
mod resolvers;
mod schema;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use dotenvy::dotenv;
    use std::env;

    dotenv().ok();

    let todo_management_service_url =
        env::var("TODO_MANAGEMENT_SERVICE_URL").expect("DATABASE_URL must be set");

    let todo_client = TodoClient::new(todo_management_service_url).await?;
    let schema = schema::create_schema(todo_client);

    let app = Router::new()
        .route("/", post(graphql_handler))
        .layer(Extension(schema));

    let listener = TcpListener::bind("localhost:3000").await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
