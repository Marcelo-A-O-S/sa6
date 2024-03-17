
use actix_web::{get, HttpResponse, Responder, Result};
use crate::services::{TServices::TServices, AcademiaServices::AcademiaServices};
use serde;
#[get("/GetAcademias")]
pub async fn get_academias_controller() ->Result<impl Responder>{
    let listaAcademia = AcademiaServices::Listar().await;
    Ok(HttpResponse::Ok().json(listaAcademia))
}
