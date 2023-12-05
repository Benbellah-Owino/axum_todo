#[allow(dead_code)]
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    extract::{Query,State},
    Json,
};
use axum_macros::debug_handler;
use crate::db::Db;
use surrealdb::sql::Thing;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo{
    pub item: Option<String>,
    pub completed: bool,
}

#[derive(Deserialize, Debug)]
pub struct CreateTodo{
    pub item:String,
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
pub struct Params{
    pub id:Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Record{
    pub id: Thing,
}

pub async fn create_todo(State(db): State<Db>, Json(input): Json<CreateTodo>) -> impl IntoResponse{
    let todo = Todo::new(input.item);

    let created: Result<Vec<Record>, surrealdb::Error> = db.unwrap().create("todo")
                                 .content(todo).await;
    dbg!(created);

    (StatusCode::CREATED).into_response()
}

pub async fn get_todo(State(db): State<Db>, ) -> impl IntoResponse{
    let todos = db.unwrap().query("SELECT * FROM type::table($table);").bind(("table","todo")).await;
    dbg!(todos);
    (StatusCode::FOUND).into_response()
}

pub async fn update_todo(State(db): State<Db>, Query(_params):Query<Params>) -> impl IntoResponse{
    let todo = db.unwrap().query("SELECT * FROM type::table($table);").bind(("table","todo")).await;

    dbg!(todo);
    (StatusCode::FOUND).into_response()
}

pub async fn delete_todo(State(db): State<Db>, Query(params): Query<Params>) -> impl IntoResponse{
    let todo : Result<std::option::Option<Record>, surrealdb::Error>= db.unwrap().delete(("person", params.id.as_deref().unwrap())).await;

    dbg!(todo);
    (StatusCode::FOUND).into_response()
}
