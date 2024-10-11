use crate::repository::TodoRepository;
use tonic::{Request, Response, Status};
pub struct TodoService {
    repository: TodoRepository,
}

pub mod todo {
    tonic::include_proto!("todo");
}

use todo::{
    todo_service_server::TodoService as TodoServiceTrait, Empty, NewTodo, Todo, TodoDeleteResponse,
    TodoId, TodoList, UpdateTodoRequest,
};

impl TodoService {
    pub fn new(repository: TodoRepository) -> Self {
        Self { repository }
    }
}

#[tonic::async_trait]
impl TodoServiceTrait for TodoService {
    async fn get_todo_by_id(&self, request: Request<TodoId>) -> Result<Response<Todo>, Status> {
        let request = request.into_inner();

        let result = self.repository.get_todo(request.id);

        if let Ok(todo) = result {
            if let Some(todo) = todo {
                let todo = Todo {
                    id: todo.id,
                    title: todo.title,
                    description: todo.description,
                    completed: todo.completed,
                    created_at: todo.created_at.to_string(),
                    updated_at: todo.updated_at.map(|x| x.to_string()),
                };
                return Ok(Response::new(todo.into()));
            } else {
                return Err(Status::not_found("Todo not found"));
            }
        } else {
            return Err(Status::internal("Internal server error"));
        }
    }

    async fn get_all_todos(&self, _request: Request<Empty>) -> Result<Response<TodoList>, Status> {
        todo!()
    }

    async fn create_new_todo(&self, request: Request<NewTodo>) -> Result<Response<Todo>, Status> {
        todo!()
    }

    async fn update_todo(
        &self,
        request: Request<UpdateTodoRequest>,
    ) -> Result<Response<Todo>, Status> {
        todo!()
    }

    async fn delete_todo_by_id(
        &self,
        request: Request<TodoId>,
    ) -> Result<Response<TodoDeleteResponse>, Status> {
        todo!()
    }
}
