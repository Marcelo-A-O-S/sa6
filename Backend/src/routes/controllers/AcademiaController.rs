use chrono::{Datelike, NaiveDate, NaiveTime};
use crate::{
    entities::{
        Academia::Academia,
        Agendamento::Agendamento,
        DataHora::DataHora
    },
    services::{
        AcademiaServices::AcademiaServices, TServices::TServices, UsuarioServices::UsuarioServices
    },
};
use actix_web::{delete, get, post, put, web, Error, HttpResponse, Responder, Result};
use super::ViewModel::{
    RequestPathId::RequestPathId,
    AcademiaRequest::AcademiaRequest,
    AgendamentoRequest::AgendamentoRequest,
    ResponseAgendamento::ResponseAgendamento
};
#[get("/Academias")]
pub async fn get_academias_controller() -> Result<impl Responder> {
    let mut academiaServices = AcademiaServices::new();
    let result = academiaServices.Listar().await;
    match result {
        Ok(listaAcademia) => Ok(HttpResponse::Ok().json(listaAcademia)),
        Err(err) => return Ok((HttpResponse::NotFound().json("Nada encontrado"))),
    }
}
#[post("/CreateAcademia")]
pub async fn create_academia(
    academiaJson: web::Json<AcademiaRequest>,
) -> Result<HttpResponse, Error> {
    let academia_body = academiaJson.into_inner();
    let mut academia_services = AcademiaServices::new();
    if academia_body.Endereco != ""
        && academia_body.NomeComercial != ""
        && academia_body.CapacidadeUsuarios != 0
    {
        let mut academia = Academia::new(
            academia_body.Id,
            academia_body.NomeComercial,
            academia_body.HorarioAbertura,
            academia_body.HorarioFechamento,
            academia_body.CapacidadeUsuarios,
            academia_body.Endereco,
        );
        if academia_body.Id == 0 {
            let result_salvar = academia_services.Salvar(academia).await;
            match result_salvar {
                Ok(()) => {
                    println!("Salvo");
                    return Ok(HttpResponse::Ok().json("Academia salva com sucesso!"));
                }
                Err(err) => {
                    println!("Ocorreu erro");
                    return Ok(
                        HttpResponse::BadRequest().json("Ocorreu um erro ao salvar o objeto")
                    );
                }
            }
        } else {
            let result_salvar = academia_services.Atualizar(academia).await;
            match result_salvar {
                Ok(()) => {
                    return Ok(HttpResponse::Ok().json("Academia salva com sucesso!"));
                }
                Err(err) => {
                    return Ok(
                        HttpResponse::BadRequest().json("Ocorreu um erro ao salvar o objeto")
                    );
                }
            }
        }
    }
    return Ok(HttpResponse::BadRequest().json("Dados inválidos"));
}
#[put("/UpdateAcademia")]
pub async fn update_academia(
    academiaJson: web::Json<AcademiaRequest>,
) -> Result<HttpResponse, Error> {
    let academia_body = academiaJson.into_inner();
    let mut academia_services = AcademiaServices::new();
    if academia_body.Endereco != ""
        && academia_body.NomeComercial != ""
        && academia_body.CapacidadeUsuarios != 0
    {
        let mut academia_update = Academia::new(
            academia_body.Id,
            academia_body.NomeComercial,
            academia_body.HorarioAbertura,
            academia_body.HorarioFechamento,
            academia_body.CapacidadeUsuarios,
            academia_body.Endereco,
        );
        if academia_body.Id != 0 {
            let result_busca = academia_services.BuscarPorId(academia_update.Id).await;
            match result_busca {
                Ok(academia) => {
                    let result_update = academia_services.Atualizar(academia_update).await;
                    match result_update {
                        Ok(()) => return Ok(HttpResponse::Ok().json("Foi atualizado com sucesso")),
                        Err(err) => {
                            return Ok(
                                HttpResponse::BadRequest().json("Erro em atualizar a entidade")
                            )
                        }
                    }
                }
                Err(err) => return Ok(HttpResponse::NotFound().json("Entidade não encontrada")),
            }
        } else {
            return Ok(HttpResponse::BadRequest().json("Dados inválidos"));
        }
    } else {
        Ok(HttpResponse::BadRequest().json("Dados inválidos"))
    }
}
#[delete("/DeleteAcademia")]
pub async fn delete_academia(
    academiaJson: web::Json<AcademiaRequest>,
) -> Result<HttpResponse, Error> {
    let academia_body = academiaJson.into_inner();
    let mut academia_services = AcademiaServices::new();
    if academia_body.Endereco != ""
        && academia_body.NomeComercial != ""
        && academia_body.CapacidadeUsuarios != 0
    {
        if academia_body.Id != 0 {
            let result_busca = academia_services.BuscarPorId(academia_body.Id).await;
            match result_busca {
                Ok((academia)) => {
                    let result_update = academia_services.Deletar(academia).await;
                    match result_update {
                        Ok(()) => {
                            return Ok(HttpResponse::Ok().json("Foi deletado com sucesso"));
                        }
                        Err(err) => {
                            return Ok(HttpResponse::BadRequest().json("Dados inválidos"));
                        }
                    }
                }
                Err(err) => {
                    return Ok(HttpResponse::NotFound().json("Dados inválidos"));
                }
            }
        } else {
            return Ok(HttpResponse::BadRequest().json("Dados inválidos"));
        }
    } else {
        return Ok(HttpResponse::Ok().json("Dados inválidos"));
    }
}
/*  #[get("/GetAgendamentosByAcademiaId/{Id}")]
pub async fn get_agendamentos_by_academia_id(path : web::Path<RequestPathId>)-> Result<HttpResponse, Error>{
    let mut academia_id = path.into_inner(); 
} */
#[post("/CreateAgendamento")]
pub async fn create_agendamento(agendamentoJson: web::Json<AgendamentoRequest>)-> Result<HttpResponse, Error>{
    let mut agendamento_body = agendamentoJson.into_inner();
    let mut academia_services = AcademiaServices::new();
    if(agendamento_body.Id == 0 ){
        if agendamento_body.AcademiaId != 0 
        && agendamento_body.Data != String::from("") 
        && agendamento_body.HorarioFechamento != NaiveTime::from_hms_opt(0, 0, 0).unwrap()
        && agendamento_body.HorarioInicial != NaiveTime::from_hms_opt(0, 0, 0).unwrap() 
        && agendamento_body.UsuarioId != 0
        {
            let parts: Vec<&str> = agendamento_body.Data.as_str().split('/').collect();
            let ano = parts[2].parse::<i32>().unwrap();
            let mes = parts[1].parse::<i32>().unwrap();
            let dia = parts[0].parse::<i32>().unwrap();
            let mut dh = DataHora{
                Ano: ano,
                Mes: mes,
                Dia: dia,
                Id: 0,
                HoraInicial: agendamento_body.HorarioInicial,
                HoraFechamento: agendamento_body.HorarioFechamento
            };
            let result_save = academia_services.salvar_agendamento(agendamento_body.AcademiaId, agendamento_body.UsuarioId, dh).await;
            if result_save.is_ok(){
                return Ok(HttpResponse::Ok().json("Agendamento salvo com sucesso"))
            }else{
                return Ok(HttpResponse::BadRequest().json("Ocorreu um problema ao salvar os dados"))
            }
            
        }else{
            return Ok(HttpResponse::BadRequest().json("Dados inválidos"))
        }
    }else{
        if agendamento_body.AcademiaId != 0 
        && agendamento_body.Data != String::from("") 
        && agendamento_body.UsuarioId != 0{
            let parts: Vec<&str> = agendamento_body.Data.as_str().split('/').collect();
            let ano = parts[2].parse::<i32>().unwrap();
            let mes = parts[1].parse::<i32>().unwrap();
            let dia = parts[0].parse::<i32>().unwrap();
            println!("{:?}", agendamento_body.HorarioInicial);
            println!("{:?}", agendamento_body.HorarioFechamento);
            let mut dh = DataHora{
                Ano: ano,
                Mes: mes,
                Dia: dia,
                Id: agendamento_body.Id,
                HoraInicial: agendamento_body.HorarioInicial,
                HoraFechamento: agendamento_body.HorarioFechamento
            };
            let result_update = academia_services.atualizar_agendamento(agendamento_body.Id, dh, agendamento_body.AcademiaId, agendamento_body.UsuarioId).await;
            match result_update{
                Ok(())=>{
                    return Ok(HttpResponse::Ok().json("Agendamento atualizado com sucesso"));
                }
                Err(err)=>{
                    println!("{:?}", err);
                    return Ok(HttpResponse::BadRequest().json("Erro ao atualzar os dados"));
                }
            }
        }else{
            return Ok(HttpResponse::BadRequest().json("Dados inválidos"))
        }
    }
    
    
} 
#[get("/GetAgendamentosByAcademiaId/{Id}")]
pub async fn get_agendamentos_by_academiaid(path : web::Path<RequestPathId>)-> Result<HttpResponse, Error>{
    let mut academia_services = AcademiaServices::new();
    let mut usuario_services = UsuarioServices::new();
    let lista_agendamentos = academia_services.buscar_agendamentos_por_academiaid(path.Id).await;
    match  lista_agendamentos {
        Ok(agendamentos)=>{
            let mut lista:Vec<ResponseAgendamento> = Vec::new(); 
            for item in agendamentos{
                let usuario = usuario_services.BuscarPorId(item.UsuarioId).await.unwrap();
                let data = academia_services.buscar_dh_por_id(item.DataHoraId).await.unwrap();
                let response_agendamento = ResponseAgendamento{
                    data,
                    usuario,
                    Id: item.Id,
                };
                lista.push(response_agendamento);
            }
            return Ok(HttpResponse::Ok().json(lista));
        }
        Err(err)=>{
            return Ok(HttpResponse::BadRequest().json("Ocorreu algum erro na tentativa de dados"));
        }
    }
}
#[get("/AcademiaGetById/{Id}")]
pub async fn get_academia_by_id(path : web::Path<RequestPathId>)-> Result<HttpResponse, Error>{
    let mut academia_services = AcademiaServices::new();
    let result_busca = academia_services.BuscarPorId(path.Id).await;
    match result_busca {
        Ok(academia) => {
            return Ok(HttpResponse::Ok().json(academia));
        }
        Err(err) => {
            return Ok(HttpResponse::NotFound().json("Usuario não encontrado"));
        }
    }
}
#[get("/GetAgendamentoById/{Id}")]
pub async fn get_agendamento_by_id(path : web::Path<RequestPathId>)-> Result<HttpResponse, Error>{
    let mut academia_services = AcademiaServices::new();
    let mut usuario_services = UsuarioServices::new();
    let result_busca = academia_services.buscar_agendamento_por_Id(path.Id).await;
    match result_busca{
        Ok(agendamento)=>{
            let mut data = academia_services.buscar_dh_por_id(agendamento.DataHoraId).await.unwrap();
            let mut usuario = usuario_services.BuscarPorId(agendamento.UsuarioId).await.unwrap();
            let agendamento_response = ResponseAgendamento{
                data,
                usuario,
                Id: agendamento.Id
            };
            return Ok(HttpResponse::Ok().json(agendamento_response))
        }
        Err(err)=>{
            println!("{:?}",err);
            return Ok(HttpResponse::BadRequest().json(""))
        }
    }
}
#[delete("/DeleteAgendamentoById/{Id}")]
pub async fn delete_agendamento_by_id(path : web::Path<RequestPathId>) -> Result<HttpResponse, Error>{
    let mut academia_services = AcademiaServices::new();
    let mut agendamento_id = path.Id;
    let result_busca = academia_services.buscar_agendamento_por_Id(agendamento_id).await;
    match result_busca{
        Ok(agendamento)=>{
            let result_delete = academia_services.deletar_agendamento_by_id(agendamento.Id).await;
            match result_delete{
                Ok(())=>{
                    return Ok(HttpResponse::Ok().json("Deletado com sucesso"));
                }
                Err(err)=>{
                    return Ok(HttpResponse::BadRequest().json("Ocorreu um erro ao tentar deletar a entidade"))

                }
            }
        }
        Err(err)=>{
            return Ok(HttpResponse::BadRequest().json("Entidade não encontrada"))
        }
    }
}