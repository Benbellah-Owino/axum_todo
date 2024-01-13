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

#[derive(Debug, Deserialize,Serialize, Clone)]
pub struct UpdateTodo{
    field: Option<String>,
    value:Option<String>,
    old_value:Option<String>,
}

pub async fn create_todo(State(db): State<Db>, Json(input): Json<CreateTodo>) -> impl IntoResponse{
    let todo = Todo::new(input.item);

    let created: Result<Vec<TodoDb>, surrealdb::Error> = db.unwrap().create("todo")
                                 .content(todo).await;
    dbg!(created.as_deref().unwrap());

    (StatusCode::CREATED).into_response()
}

pub async fn get_todo(State(db): State<Db> ) -> impl IntoResponse{
    
    let sql = "SELECT id, item, completed FROM todo ORDER BY completed;";
    let ress = db.unwrap().query(sql).await;

    let mut todos:surrealdb::Response = ress.unwrap();
    
    let todos:Result<Vec<TodoDb>, surrealdb::Error> = todos.take(0);
    dbg!(&todos); 
    match todos{
        Ok(r) => {
            let mut res: Vec<SendTodo>= Vec::new();
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
            return (StatusCode::OK, Json(json!({"todos":res}))).into_response();
        },
        Err(e) => {
            dbg!(e);  
            return (StatusCode::NOT_FOUND).into_response();
        }
        
    }
    }
#[derive(Debug, Serialize)]
struct Item{
    item:Option<String>
}

#[derive(Debug, Serialize)]
struct Completed{
    completed: bool
}

//#[axum_macros::debug_handler]
pub async fn update_todo(State(db): State<Db>, Path(todo):Path<String>, Json(input):Json<UpdateTodo>) -> impl IntoResponse{
    
    let true_v = String::from("true");
    let false_v = String::from("false");
    match input.field.unwrap().as_str() {
        "item" => {
            let update_item = input.value.clone().unwrap();
            //let sql = "UPDATE 
            let new_item: Item = Item{
                item: Some(update_item),
            };
            
            // Result<Option<TodoDb>,surrealdb::Error> 
            let updated: Option<TodoDb> = db.unwrap().update(("todo",todo)).merge(new_item).await.unwrap();
            return (StatusCode::FOUND, Json(json!({"todos": updated}))).into_response()
        },
        "completed" =>{
            let update_item = input.value.clone().unwrap();
            let mut updated: Option<TodoDb> = None;
            let mut new_item:Completed = Completed{
                completed:true,
            };

            if update_item  == true_v{
                updated = db.unwrap().update(("todo",todo)).merge(new_item).await.unwrap();
            }else if update_item == false_v{
                new_item.completed = false;
                updated =db.unwrap().update(("todo",todo)).merge(new_item).await.unwrap();
            } 

            return (StatusCode::FOUND, Json(json!({"todos": updated}))).into_response()

        }

        _ => {
            println!("Neigh");
            return (StatusCode::FOUND).into_response()

        }
    }
}

pub async fn delete_todo(State(db): State<Db>,Path(id):Path<String>) -> impl IntoResponse{
    println!("{id}");
        
    let todo : Option<Todo> = db.unwrap().delete(("todo", id)).await.unwrap();

    //dbg!(todo);
    (StatusCode::FOUND, Json(json!({"deleted":todo})).into_response())
}


