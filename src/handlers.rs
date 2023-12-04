#[allow(dead_code)]
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    extract::{Path,Query,State},
    Json,
};
use axum_macros::debug_handler;
use serde_json::{json};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use crate::db::Db;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo{
    pub item: Option<String>,
    completed: bool,
}

#[derive(Deserialize, Debug)]
struct CreateTodo{
    item:String,
}

impl Todo{
    pub fn new(item: String) -> Self{
        Todo{
            item: Some(item),
            completed: true,
        }
    }
}


#[derive(Debug, Deserialize)]
struct Params{
    id:Option<String>,
}


#[debug_handler]
pub async fn create_todo(State(db): State<Db>, Json(input): Json<CreateTodo>) -> impl IntoResponse{
    let todo = Todo::new(input.item);

    let created = db.unwrap().create("todo")
                                 .content(todo);
    dbg!(created);

    (StatusCode::CREATED).into_response()
}

#[debug_handler]
pub async fn get_todo(State(db): State<Db>, ) -> impl IntoResponse{
    let todos = db.unwrap().query("SELECT * FROM type::table($table) GROUP BY completed;").bind(("table","todo"));
    dbg!(todos);
    (StatusCode::FOUND).into_response();
}

#[debug_handler]
pub async fn update_todo(State(db): State<Db>, Query(params):Query<Params>) -> impl IntoResponse{
    let todo = db.unweap().query("SELECT * FROM type::table($table);").bind(("table","todo")).await;

    dbg!(todo);
    (StatusCode::FOUND).into_response()
}

#[debug_handler]
pub async fn delete_todo(State(db): State<Db>, Query(params): Query<Params>) -> impl IntoResponse{
    let todo = db.unwrap().delete(("person", params.id.as_deref().unwrap()));

    dbg!(todo);
    (StatusCode::FOUND).into_response()

}
