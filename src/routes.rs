use crate::handlers::{create_todo, delete_todo, get_todo, update_todo};
use axum::routing::{get, post, patch, delete };
use axum::{ Router, response::IntoResponse};
use crate::db::{connect_db, Error, Db};
use axum_macros::debug_handler;

pub fn routes(db: Db) -> Router{
    dbg!(db.clone());
    Router::new()
        .route("/todo/:id", patch(delete_todo).delete(update_todo))
        .route("/todo", post(create_todo).get(get_todo))
        .with_state(db)
}
