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
    let todo_client = TodoClient::new("http://localhost:50051".to_string()).await?;
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
