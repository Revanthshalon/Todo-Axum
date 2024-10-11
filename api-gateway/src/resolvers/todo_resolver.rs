use crate::clients::todo::{NewTodo, UpdateTodoRequest};
use crate::clients::todo_client::TodoClient;
use async_graphql::{Result, SimpleObject};
use chrono::{DateTime, Utc};

#[derive(SimpleObject)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct TodoResolver;

impl TodoResolver {
    pub async fn get_all_todos(client: &TodoClient) -> Result<Vec<Todo>> {
        let response = client.get_all_todos().await?;
        Ok(response
            .into_inner()
            .todos
            .into_iter()
            .map(|t| Todo {
                id: t.id,
                title: t.title,
                description: t.description,
                completed: t.completed,
                created_at: t.created_at.parse::<DateTime<Utc>>().unwrap(),
                updated_at: t.updated_at.map(|t| t.parse::<DateTime<Utc>>().unwrap()),
            })
            .collect())
    }

    pub async fn get_todo_by_id(client: &TodoClient, id: i32) -> Result<Option<Todo>> {
        let response = client.get_todo_by_id(id).await;
        match response {
            Ok(response) => {
                let response = response.into_inner();
                Ok(Some(Todo {
                    id: response.id,
                    title: response.title,
                    description: response.description,
                    completed: response.completed,
                    created_at: response.created_at.parse::<DateTime<Utc>>().unwrap(),
                    updated_at: response
                        .updated_at
                        .map(|t| t.parse::<DateTime<Utc>>().unwrap()),
                }))
            }
            Err(_) => Ok(None),
        }
    }

    pub async fn create_new_todo(
        client: &TodoClient,
        title: String,
        description: Option<String>,
    ) -> Result<Todo> {
        let new_todo = NewTodo { title, description };
        let response = client.create_new_todo(new_todo).await?.into_inner();

        Ok(Todo {
            id: response.id,
            title: response.title,
            description: response.description,
            completed: response.completed,
            created_at: response.created_at.parse::<DateTime<Utc>>().unwrap(),
            updated_at: response
                .updated_at
                .map(|t| t.parse::<DateTime<Utc>>().unwrap()),
        })
    }

    pub async fn update_todo(
        client: &TodoClient,
        id: i32,
        title: Option<String>,
        description: Option<String>,
        completed: Option<bool>,
    ) -> Result<Todo> {
        let update_todo = UpdateTodoRequest {
            id,
            title,
            description,
            completed,
        };
        let response = client.update_todo(update_todo).await;
        match response {
            Ok(response) => {
                let response = response.into_inner();
                Ok(Todo {
                    id: response.id,
                    title: response.title,
                    description: response.description,
                    completed: response.completed,
                    created_at: response.created_at.parse::<DateTime<Utc>>().unwrap(),
                    updated_at: response
                        .updated_at
                        .map(|t| t.parse::<DateTime<Utc>>().unwrap()),
                })
            }
            Err(e) => Err(e.into()),
        }
    }

    pub async fn delete_todo_by_id(client: &TodoClient, id: i32) -> Result<bool> {
        let response = client.delete_todo_by_id(id).await?;
        Ok(response.into_inner().success)
    }
}
