use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use crate::handlers::Todo;

pub type Db = Result<Surreal<Client>,Error>;


async fn connect_db() -> Result<Surreal<Client>, Error>{
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
    
    db.signin(Root{
        username: "root",
        password: "root",
    });

    db.use_ns("todo").use_db("todo").await;

    return OK(db);
}
#[derive(Debug, Clone)]
pub enum Error{
    DbConnectionError,
}
