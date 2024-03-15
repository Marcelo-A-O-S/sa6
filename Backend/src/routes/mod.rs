use actix_web::{App, HttpServer};
pub mod controllers;

use crate::routes::controllers::AcademiaController::academia_controller;



pub async fn app_server() {
    let _server = HttpServer::new(||{
        App::new()
        .service(academia_controller)
    });
}
