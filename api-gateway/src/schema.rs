use crate::clients::todo_client::TodoClient;
use crate::resolvers::todo_resolver::{Todo, TodoResolver};
use async_graphql::{Context, EmptySubscription, Object, Result, Schema};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    pub async fn todos(&self, ctx: &Context<'_>) -> Result<Vec<Todo>> {
        let todo_client = ctx.data::<TodoClient>()?;
        TodoResolver::get_all_todos(todo_client).await
    }

    pub async fn todo_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Todo>> {
        let todo_client = ctx.data::<TodoClient>()?;
        TodoResolver::get_todo_by_id(todo_client, id).await
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_new_todo(
        &self,
        ctx: &Context<'_>,
        title: String,
        description: Option<String>,
    ) -> Result<Todo> {
        let todo_client = ctx.data::<TodoClient>()?;
        TodoResolver::create_new_todo(todo_client, title, description).await
    }

    async fn update_todo(
        &self,
        ctx: &Context<'_>,
        id: i32,
        title: Option<String>,
        description: Option<String>,
        completed: Option<bool>,
    ) -> Result<Todo> {
        let todo_client = ctx.data::<TodoClient>()?;
        TodoResolver::update_todo(todo_client, id, title, description, completed).await
    }

    async fn delete_todo_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let todo_client = ctx.data::<TodoClient>()?;
        let response = todo_client.delete_todo_by_id(id).await?;
        Ok(response.into_inner().success)
    }
}

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(todo_client: TodoClient) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(todo_client)
        .finish()
}
