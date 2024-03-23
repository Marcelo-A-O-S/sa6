
mod repository;
mod entities;
mod routes;
mod schema;
mod connection;
mod utils;
mod services;
mod generics;

use routes::app_server;

#[actix_web::main]
async fn main()-> std::io::Result<()>{
   app_server().await
    
}