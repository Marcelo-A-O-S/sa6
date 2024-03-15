use actix_web::{web, App, HttpServer};
pub mod controllers;

use crate::routes::controllers::AcademiaController::GetAllAcademias;



pub async fn AppServer(){
    let server = HttpServer::new(||{
        App::new()
        .service(GetAllAcademias)
    });
}