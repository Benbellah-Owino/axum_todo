//! Provides a RESTful web server managing some Todos.
//!
//! API will be:
//!
//! - `GET /todos`: return a JSON list of Todos.
//! - `POST /todos`: create a new Todo.
//! - `PATCH /todos/:id`: update a specific Todo.
//! - `DELETE /todos/:id`: delete a specific Todo.
//!
//! Run with
//!
//! ```not_rust
//! cargo run -p example-todos
//! ```
#[allow(unused_imports)]
use axum::{
    error_handling::HandleErrorLayer,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json,
};


use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use todos_app::routes;
use todos_app::db::connect_db;
use axum::{ Router};
#[tokio::main]
async fn main(){
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_|"todos_app=debug, tower_http=debug".into()),
            )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = connect_db().await;
    let app = Router::new()
                .nest("/todos",routes::routes())
                .with_state(db)
                .route("/",get(hello_todos));
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
            .await
            .unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn hello_todos() -> &'static str{
    "Hello Todo"
}
