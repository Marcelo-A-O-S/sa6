use actix_web::{get, HttpResponse, Responder};
use crate::services::UsuarioServices::UsuarioServices;
#[get("/GetUsuarios")]
async fn get_usuarios()->impl Responder{
    HttpResponse::Ok().body("Hello World!")
}