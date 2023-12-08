#[allow(dead_code)]
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    extract::{Query,State,Path},
    Json,
};
use serde_json::json;
use axum_macros::debug_handler;
use crate::db::Db;
use surrealdb::sql::{Thing, thing, Object, Value};
use std::collections::BTreeMap;
use surrealdb::sql::Kind;
use surrealdb::dbs::Response;
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
            completed: false,
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct Params{
    pub id:Option<String>,
}

#[derive(Debug, Deserialize,Serialize, Clone)]
pub struct TodoDb{
    pub id: Thing,
    pub item: Option<String>,
    pub completed:bool,
}

#[derive(Debug, Deserialize)]
pub struct Record{
    pub id: Thing,
    pub item: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct SendTodo{
    pub id: String,
    pub item: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct UpdateTodo{
    field: String,
    value: String,
}

pub async fn create_todo(State(db): State<Db>, Json(input): Json<CreateTodo>) -> impl IntoResponse{
    let todo = Todo::new(input.item);

    let created: Result<Vec<TodoDb>, surrealdb::Error> = db.unwrap().create("todo")
                                 .content(todo).await;
    dbg!(created.as_deref().unwrap());

    (StatusCode::CREATED).into_response()
}

pub async fn get_todo(State(db): State<Db>, ) -> impl IntoResponse{
    //let todos = db.unwrap().query("SELECT id, item, completed FROM type::table($table) ORDER BY completed;").bind(("table","todo")).await;
    //dbg!(todos.unwrap());
    //
    let sql = "SELECT id, item, completed FROM todo ORDER BY completed;";
    let ress = db.unwrap().query(sql).await;

    let mut todos:surrealdb::Response = ress.unwrap();
    
    let todos:Result<Vec<TodoDb>, surrealdb::Error> = todos.take(0);
    
    match todos{
        Ok(r) => {
            let mut res: Vec<SendTodo>= Vec::new();
            dbg!(&r);
            let r2 = r.clone();
            for i in r2.iter(){
                
                let item = i.item.as_ref().unwrap().to_string();
                let id = format!("{}:{}", i.id.tb, i.id.id);

                let todo = SendTodo{
                    id,
                    item,
                    completed: i.completed,
                };
                res.push(todo);
            }
            dbg!(&res);
            return (StatusCode::FOUND, Json(json!({"todos":res}))).into_response();
        },
        Err(e) => {
            dbg!(e);
            return (StatusCode::NOT_FOUND).into_response();
        }
        
    }
    }

pub async fn update_todo(State(db): State<Db>, Path(todo):Path<String>) -> impl IntoResponse{
    println!("{:?}", todo);
    /*let todo = db.unwrap().query("SELECT * FROM type::table($table);").bind(("table","todo")).await;

    dbg!(todo);*/
    (StatusCode::FOUND).into_response()
}

pub async fn delete_todo(State(db): State<Db>,Path(todo):Path<String>) -> impl IntoResponse{
        println!("{todo}");
        
    //let todo : Option<Todo> = db.unwrap().delete(("todo", params.id.as_deref().unwrap())).await;

    //dbg!(todo);
    (StatusCode::FOUND).into_response()
}


