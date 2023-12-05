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
use surrealdb::sql::{Thing, thing, Object, Value};

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

#[derive(Debug, Deserialize)]
pub struct Record{
    pub id: Thing,
    pub item: String,
    pub completed: bool,
}

pub async fn create_todo(State(db): State<Db>, Json(input): Json<CreateTodo>) -> impl IntoResponse{
    let todo = Todo::new(input.item);

    let created: Result<Vec<Record>, surrealdb::Error> = db.unwrap().create("todo")
                                 .content(todo).await;
    dbg!(created.as_deref().unwrap());

    (StatusCode::CREATED).into_response()
}

pub async fn get_todo(State(db): State<Db>, ) -> impl IntoResponse{
    //let todos = db.unwrap().query("SELECT id, item, completed FROM type::table($table) ORDER BY completed;").bind(("table","todo")).await;
    //dbg!(todos.unwrap());
    //
    let sql = "SELECT id, item, completed FROM type::table($table) ORDER BY completed;";
    let ress = db.unwrap().query(sql).bind(("table", "todo")).await;
    for object in into_iter_objects(ress)?{
        println!("record {}", object);
    };
    (StatusCode::FOUND).into_response()
}

pub async fn update_todo(State(db): State<Db>, Query(_params):Query<Params>) -> impl IntoResponse{
    let todo = db.unwrap().query("SELECT * FROM type::table($table);").bind(("table","todo")).await;

    dbg!(todo);
    (StatusCode::FOUND).into_response()
}

pub async fn delete_todo(State(db): State<Db>, Query(params): Query<Params>) -> impl IntoResponse{
    let id = params.id.as_deref().unwrap();
    println!("{id}");
        
    //let todo : Option<Todo> = db.unwrap().delete(("todo", params.id.as_deref().unwrap())).await;

    //dbg!(todo);
    (StatusCode::FOUND).into_response()
}

fn into_iter_objects(ress: Vec<Response>) -> Result<impl Iterator<Item = Result<Object>, Error>, Error> {
	let res = ress.into_iter().next().map(|rp| rp.result).transpose()?;

	match res {
		Some(Value::Array(arr)) => {
			let it = arr.into_iter().map(|v| match v {
				Value::Object(object) => Ok(object),
				_ => Err(anyhow!("A record was not an Object")),
			});
			Ok(it)
		}
		_ => Err(anyhow!("No records found.")),
	}
}
