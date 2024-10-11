use crate::clients::todo::todo_service_client::TodoServiceClient;
use crate::clients::todo::{
    Empty, NewTodo, Todo, TodoDeleteResponse, TodoId, TodoList, UpdateTodoRequest,
};
use tonic::transport::Channel;
use tonic::{Response, Status};

pub struct TodoClient {
    client: TodoServiceClient<Channel>,
}

impl TodoClient {
    pub async fn new(addr: String) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::from_shared(addr)?.connect().await?;
        let client = TodoServiceClient::new(channel);
        Ok(Self { client })
    }

    pub async fn get_all_todos(&self) -> Result<Response<TodoList>, Status> {
        self.client.clone().get_all_todos(Empty {}).await
    }

    pub async fn get_todo_by_id(&self, id: i32) -> Result<Response<Todo>, Status> {
        self.client.clone().get_todo_by_id(TodoId { id }).await
    }

    pub async fn create_new_todo(&self, todo: NewTodo) -> Result<Response<Todo>, Status> {
        self.client.clone().create_new_todo(todo).await
    }

    pub async fn update_todo(&self, todo: UpdateTodoRequest) -> Result<Response<Todo>, Status> {
        self.client.clone().update_todo(todo).await
    }

    pub async fn delete_todo_by_id(&self, id: i32) -> Result<Response<TodoDeleteResponse>, Status> {
        self.client.clone().delete_todo_by_id(TodoId { id }).await
    }
}
