
use actix_web::{get, HttpResponse, Responder};


#[get("/Academias")]
pub async fn academia_controller() ->  impl Responder {
    HttpResponse::Ok().body("Hello World!")
}
