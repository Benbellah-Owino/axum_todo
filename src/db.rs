use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

pub type Db = Result<Surreal<Client>,Error>;


pub async fn connect_db() -> Result<Surreal<Client>, Error>{
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
    
    db.signin(Root{
        username: "root",
        password: "root",
    }).await;

    db.use_ns("todo").use_db("todo").await;

    return Ok(db);
}
#[derive(Debug, Clone)]
pub enum Error{
    DbConnectionError,
}
