use actix_web::{App, HttpServer};
pub mod controllers;
use controllers::academiaController::get_academias_controller;

pub async fn app_server() -> std::io::Result<()> {
    let server = HttpServer::new(||{
        App::new()
        .service(get_academias_controller)
    })
    .bind(("127.0.0.1",8080))?
    .run();
    server.await
}




