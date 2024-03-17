
use actix_web::{get, post, web, Error, HttpResponse, Responder, Result};
use crate::services::{TServices::TServices, AcademiaServices::AcademiaServices};
use super::VIewModel::AcademiaRequest::AcademiaRequest;

#[get("/Academias")]
pub async fn get_academias_controller() ->Result<impl Responder>{
    let academiaServices = AcademiaServices::new();
    let listaAcademia = academiaServices.Listar().await;
    Ok(HttpResponse::Ok().json(listaAcademia))
}
#[post("/CreateAcademia")]
pub async fn create_academia(academiaJson: web::Json<AcademiaRequest>)-> Result<HttpResponse, Error>{
    let academiaBody = academiaJson.into_inner();
    let academiaServices = AcademiaServices::new();
    if academiaBody.Endereco != "" && academiaBody.NomeComercial != ""{
        return Ok(HttpResponse::Ok().json(academiaBody))
    }
    return Ok(HttpResponse::BadRequest().json("Dados inválidos"))
}
