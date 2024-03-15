
use actix_web::{body::BoxBody, get, HttpResponse, HttpResponseBuilder, Responder};

#[get("/Academias")]
pub async fn GetAllAcademias() ->  impl Responder {
    HttpResponse::Ok().body("Hello World!")
}
