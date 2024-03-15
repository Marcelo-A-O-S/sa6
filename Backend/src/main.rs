
use actix_web::{web, App, HttpServer, Responder};
mod repository;
mod entities;
mod routes;
mod schema;
mod connection;
use routes::app_server;
use entities::Usuarios::Usuario;

#[actix_web::main]
async fn main()-> std::io::Result<()>{
    app_server().await
}


