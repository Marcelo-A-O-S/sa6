use actix_web::{App, HttpServer, web};
use actix_cors::Cors;
pub mod controllers;
use controllers::academiaController::get_academias_controller;
use controllers::UsuariosController::{create_usuarios,get_usuarios};
/* pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_academias_controller)
    .service(create_usuarios);
} */


pub async fn app_server() -> std::io::Result<()> {
    let server = HttpServer::new(||{
        let cors = Cors::permissive();
        App::new()
        .wrap(cors)
        .service(get_academias_controller)
        .service(create_usuarios)
        .service(get_usuarios)
    })
    .bind(("127.0.0.1",8080))?
    .run();
    server.await
}


