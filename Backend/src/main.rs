use actix_web::{web, App, HttpServer, Responder};
mod repository;
mod entities;
mod routes;
mod schema;
mod connection;
use routes::AppServer;

use diesel::prelude::*;

#[actix_web::main]
async fn main()-> std::io::Result<()>{
    AppServer().await
}


