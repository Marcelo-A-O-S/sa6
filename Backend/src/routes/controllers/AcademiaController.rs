
use actix_web::{get, post,put, delete, web, Error, HttpResponse, Responder, Result};
use crate::{entities::Academia::Academia, services::{AcademiaServices::AcademiaServices, TServices::TServices}};
use super::VIewModel::AcademiaRequest::AcademiaRequest;

#[get("/Academias")]
pub async fn get_academias_controller() ->Result<impl Responder>{
    let mut academiaServices = AcademiaServices::new();
    let result = academiaServices.Listar().await;
    match result {
        Ok(listaAcademia)=>{
            Ok(HttpResponse::Ok().json(listaAcademia))
        }
        Err(err)=>{
            return Ok((HttpResponse::NotFound().json("Nada encontrado")))
        }
    }
}
#[post("/CreateAcademia")]
pub async fn create_academia(academiaJson: web::Json<AcademiaRequest>)-> Result<HttpResponse, Error>{
    let academia_body = academiaJson.into_inner();
    let mut academia_services = AcademiaServices::new();
    if academia_body.Endereco != "" && academia_body.NomeComercial != "" && academia_body.CapacidadeUsuarios != 0 {
        let mut academia = Academia::new(academia_body.Id, academia_body.NomeComercial, academia_body.HorarioAbertura, academia_body.HorarioFechamento, academia_body.CapacidadeUsuarios, academia_body.Endereco);
        if academia_body.Id == 0 {
            let result_salvar = academia_services.Salvar(academia).await;
            match result_salvar{
                Ok(())=>{
                    return Ok(HttpResponse::Ok().json("Academia salva com sucesso!"));
                }
                Err(err)=>{
                    return Ok(HttpResponse::BadRequest().json("Ocorreu um erro ao salvar o objeto"));
                }
            }
        }else{
            let result_salvar = academia_services.Atualizar(academia).await;
            match result_salvar{
                Ok(())=>{
                    return Ok(HttpResponse::Ok().json("Academia salva com sucesso!"));
                }
                Err(err)=>{
                    return Ok(HttpResponse::BadRequest().json("Ocorreu um erro ao salvar o objeto"));
                }
            }
        }
    }
    return Ok(HttpResponse::BadRequest().json("Dados inválidos"))
}
#[put("/UpdateAcademia")]
pub async fn update_academia(academiaJson: web::Json<AcademiaRequest>)-> Result<HttpResponse, Error>{
    let academia_body = academiaJson.into_inner();
    let mut academia_services = AcademiaServices::new();
    if academia_body.Endereco != "" && academia_body.NomeComercial != "" && academia_body.CapacidadeUsuarios != 0 {
        let mut academia_update = Academia::new(academia_body.Id, academia_body.NomeComercial, academia_body.HorarioAbertura, academia_body.HorarioFechamento, academia_body.CapacidadeUsuarios, academia_body.Endereco);
        if academia_body.Id != 0 {
            let result_busca = academia_services.BuscarPorId(academia_update.Id).await;
            match result_busca{
                Ok(academia)=>{
                     let result_update = academia_services.Atualizar(academia).await;
                     match result_update{
                        Ok(())=>{
                            return Ok(HttpResponse::Ok().json("Foi atualizado com sucesso"))
                        }
                        Err(err)=>{
                            return Ok(HttpResponse::BadRequest().json("Erro em atualizar a entidade"))
                        }
                     }
                }
                Err(err)=>{
                    return Ok(HttpResponse::NotFound().json("Entidade não encontrada"))
                }
            }
        }else{
            return Ok(HttpResponse::BadRequest().json("Dados inválidos"))
        }
    }else{
        Ok(HttpResponse::BadRequest().json("Dados inválidos"))
    }
}
#[delete("/DeleteAcademia")]
pub async fn delete_academia(academiaJson: web::Json<AcademiaRequest>)-> Result<HttpResponse, Error>{
    let academia_body = academiaJson.into_inner();
    let mut academia_services = AcademiaServices::new();
    if academia_body.Endereco != "" && academia_body.NomeComercial != "" && academia_body.CapacidadeUsuarios != 0 {
<<<<<<< HEAD
        if academia_body != 0 {
            let result_busca = academia_services.BuscarPorId(academia_body.Id).await;
        }else{

        }
    }else{
        Ok(HttpResponse::Ok().json("Dados inválidos"))
=======
        
>>>>>>> origin/main
    }
}