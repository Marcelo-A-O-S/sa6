<<<<<<< HEAD
use actix_web::{web, App, HttpServer, Responder};
mod repository;
mod entities;
mod routes;
mod schema;
mod connection;
use routes::AppServer;
=======



mod repository;
mod entities;
mod routes;

use entities::Usuarios::Usuario;

fn main(){
    let user = Usuario{
        cpf: String::from("Teste"),
        nome: String::from("Henrique")
    };
>>>>>>> e096852d21975b9a1438409ed6f96634dd56c077

use diesel::prelude::*;

#[actix_web::main]
async fn main()-> std::io::Result<()>{
    AppServer().await
}


