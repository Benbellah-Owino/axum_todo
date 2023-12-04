use crate::handlers::{create_todo, delete_todo, get_todo, update_todo};

use axum::routing::{delete, post, patch, };
use axum::{Json, Router,extract::{Query, State}};

pub fn routes() -> Router{
    Router::new()
        .route("/todo", post(create_todo).get(get_todo))
        .route("/todo/:id",delete(delete_todo).patch(update_todo))
}
