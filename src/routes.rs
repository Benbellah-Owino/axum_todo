use crate::handlers::{create_todo, delete_todo, get_todo, update_todo};
use axum::routing::{post,self };
use axum::routing::{delete, patch};
use axum::Router;
use crate::db:: Db;


pub fn routes(db: Db) -> Router{
    
    Router::new()
        .route("/todo", post(create_todo).get(get_todo))
        .route("/todo/:id", delete(delete_todo).patch(update_todo))
        .with_state(db)
}
