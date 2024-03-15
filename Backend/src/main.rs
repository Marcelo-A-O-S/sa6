use actix_web::{web, App, HttpServer, Responder};
use dotenvy::*;
use diesel::*;
mod repository;
mod entities;
mod routes;
use entities::Academia::Academia;
use entities::Usuarios::Usuario;
use routes::AppServer;
fn main(){
    let user = Usuario{
        cpf: String::from("Teste"),
        nome: String::from("Henrique")
    };

    println!("Nome do meliante: {:?}", user.nome);
}


