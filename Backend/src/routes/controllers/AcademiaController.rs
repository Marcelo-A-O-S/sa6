
use actix_web::{get, HttpResponse, Responder};


#[get("/GetAcademias")]
pub async fn get_academias_controller() ->  impl Responder {
    HttpResponse::Ok().body("Hello World!")
}
