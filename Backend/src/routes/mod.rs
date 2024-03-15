use actix_web::{App, HttpServer};
pub mod controllers;

<<<<<<< HEAD
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
=======
use crate::routes::controllers::AcademiaController::academia_controller;



pub async fn app_server() {
    let _server = HttpServer::new(||{
        App::new()
        .service(academia_controller)
    });
}
>>>>>>> e096852d21975b9a1438409ed6f96634dd56c077
