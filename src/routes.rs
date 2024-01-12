use crate::handlers::{create_todo, delete_todo, get_todo, update_todo};
use axum::{routing::*,Router};
use surrealdb::Surreal;
use crate::db;
use surrealdb::engine::remote::ws::Client;

pub fn routes() -> Router<Result<Surreal<Client>, db::Error>>{    
    Router::new()
        .route("/todo", post(create_todo).get(get_todo))
        .route("/todo/:id", delete(delete_todo).patch(update_todo))
}
