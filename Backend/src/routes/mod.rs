use actix_web::{web, App, HttpServer};
pub mod controllers;

use controllers::academiaController::GetAllAcademias;
pub async fn AppServer() -> std::io::Result<()> {
    let server = HttpServer::new(||{
        App::new()
        .service(GetAllAcademias)
    })
    .bind(("127.0.0.1",8080))?
    .run();
    server.await
}