use crate::handlers::{create_todo, delete_todo, get_todo, update_todo};
use axum::routing::{delete, post };
use axum::{ Router, response::IntoResponse};
use crate::db::connect_db;
use axum_macros::debug_handler;

pub async fn routes() -> Router{
    let db = connect_db().await;
    dbg!(db.clone());
    Router::new()
        .route("/todo", post(create_todo).get(get_todo))
        .route("/todo/:id",delete(delete_todo))
        .with_state(db)
}
