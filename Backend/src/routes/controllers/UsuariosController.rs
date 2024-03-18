use crate::{
    entities::Usuarios::Usuario,
    services::{TServices::TServices, UsuarioServices::UsuarioServices},
};
use actix_web::{delete, get, post, put, web, web::*, Error, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
#[derive(Deserialize, Serialize)]
struct UsuarioRequest {
    Id: i32,
    nome: String,
    CPF: String,
}
#[derive(Deserialize)]
struct UsuarioPathId {
    Id: i32,
}
#[get("/Usuarios")]
async fn get_usuarios() -> Result<HttpResponse, Error> {
    let mut usuarioServices = UsuarioServices::new();
    let result = usuarioServices.Listar().await;
    match result {
        Ok(listaUsuarios) => Ok(HttpResponse::Ok().json(listaUsuarios)),
        Err(err) => return Ok((HttpResponse::NotFound().json("Nada encontrado"))),
    }
}
#[post("/CreateUsuario")]
async fn create_usuarios(usuarioJson: web::Json<UsuarioRequest>) -> Result<HttpResponse, Error> {
    let usuarioBody = usuarioJson.into_inner();
    let mut usuario_services = UsuarioServices::new();
    if usuarioBody.nome != "" && usuarioBody.CPF != "" {
        let mut usuario = Usuario::new(usuarioBody.Id, usuarioBody.nome, usuarioBody.CPF);
        let result_validacao = usuario.validar_cpf();
        if result_validacao == true {
            if (usuarioBody.Id == 0) {
                usuario_services.Salvar(usuario).await;
                return Ok(HttpResponse::Ok().json("Usuario salvo com sucesso!"));
            } else {
                let result_buscar = usuario_services.BuscarPorId(usuarioBody.Id).await;
                match result_buscar {
                    Ok(busca) => {
                        let result_atualizar = usuario_services.Salvar(usuario).await;
                        match result_atualizar {
                            Ok(()) => {
                                return Ok(
                                    HttpResponse::Ok().json("Usuario atualizado com sucesso!")
                                );
                            }
                            Err(err) => {
                                return Ok(
                                    (HttpResponse::BadRequest().json("Erro ao Atualizar dado"))
                                )
                            }
                        }
                    }
                    Err(err) => {
                        return Ok(
                            (HttpResponse::NotFound().json("Dado não encontrado para atualização"))
                        )
                    }
                }
            }
        } else {
            return Ok(HttpResponse::BadRequest().json("Cpf inválido"));
        }
    }
    return Ok(HttpResponse::BadRequest().json("Campos inválidos"));
}
#[put("/UpdateUsuario")]
async fn update_usuario(usuarioJson: web::Json<UsuarioRequest>) -> Result<HttpResponse, Error> {
    let usuarioBody = usuarioJson.into_inner();
    let mut usuario_services = UsuarioServices::new();
    if usuarioBody.nome != "" && usuarioBody.CPF != "" {
        let result_search = usuario_services.BuscarPorId(usuarioBody.Id).await;
        match result_search {
            Ok(usuario) => {
                let usuario = Usuario::new(usuarioBody.Id, usuarioBody.nome, usuarioBody.CPF);
                let result_delete = usuario_services.Atualizar(usuario).await;
                match result_delete {
                    Ok(()) => return Ok(HttpResponse::Ok().json("Atualizado com sucesso")),
                    Err(err) => {
                        return Ok((HttpResponse::BadRequest().json("Erro ao Atualizar dado")))
                    }
                }
            }
            Err(err) => {
                return Ok(HttpResponse::NotFound().json("Usuario não encontrado"));
            }
        }
    }
    return Ok((HttpResponse::BadRequest().json("Campos inválidos")));
}
#[delete("/DeleteUsuario")]
async fn delete_usuario(usuarioJson: web::Json<UsuarioRequest>) -> Result<HttpResponse, Error> {
    let usuarioBody = usuarioJson.into_inner();
    let mut usuario_services = UsuarioServices::new();
    if usuarioBody.nome != "" && usuarioBody.CPF != "" {
        let result_search = usuario_services.BuscarPorId(usuarioBody.Id).await;
        match result_search {
            Ok(usuario) => {
                let usuario = Usuario::new(usuarioBody.Id, usuarioBody.nome, usuarioBody.CPF);
                let result_delete = usuario_services.Deletar(usuario).await;
                match result_delete {
                    Ok(()) => return Ok(HttpResponse::Ok().json("Deletado com sucesso")),
                    Err(err) => {
                        return Ok((HttpResponse::BadRequest().json("Erro ao deletar dado")))
                    }
                }
            }
            Err(err) => {
                return Ok(HttpResponse::NotFound().json("Usuario não encontrado"));
            }
        }
    }
    return Ok((HttpResponse::BadRequest().json("Campos inválidos")));
}
#[post("/DeleteUsuario")]
async fn post_delete_usuario(
    usuarioJson: web::Json<UsuarioRequest>,
) -> Result<HttpResponse, Error> {
    let usuarioBody = usuarioJson.into_inner();
    let mut usuario_services = UsuarioServices::new();
    if usuarioBody.nome != "" && usuarioBody.CPF != "" {
        let result_search = usuario_services.BuscarPorId(usuarioBody.Id).await;
        match result_search {
            Ok(usuario) => {
                let usuario = Usuario::new(usuarioBody.Id, usuarioBody.nome, usuarioBody.CPF);
                let result_delete = usuario_services.Deletar(usuario).await;
                match result_delete {
                    Ok(()) => return Ok(HttpResponse::Ok().json("Deletado com sucesso")),
                    Err(err) => {
                        return Ok((HttpResponse::BadRequest().json("Erro ao deletar dado")))
                    }
                }
            }
            Err(err) => {
                return Ok(HttpResponse::NotFound().json("Usuario não encontrado"));
            }
        }
    }
    return Ok((HttpResponse::BadRequest().json("Campos inválidos")));
}
#[delete("/DeleteById/{Id}")]
async fn delete_by_id(param_path: web::Path<UsuarioPathId>) -> Result<HttpResponse, Error> {
    let mut usuario_services = UsuarioServices::new();
    let result_busca = usuario_services.BuscarPorId(param_path.Id).await;
    match result_busca {
        Ok(usuario) => {
            let result_delete = usuario_services.Deletar(usuario).await;
            match result_delete {
                Ok(()) => {
                    return Ok(HttpResponse::Ok().json("Usuario deletado com sucesso"));
                }
                Err(err) => {
                    return Ok(
                        HttpResponse::BadRequest().json("Ocorreu algum erro ao deletar o usuário")
                    );
                }
            }
        }
        Err(err) => {
            return Ok(HttpResponse::NotFound().json("Usuario não encontrado"));
        }
    }
}
#[get("/GetById/{id}")]
async fn get_by_id(param_path: web::Path<UsuarioPathId>) -> Result<HttpResponse, Error> {
    let mut usuario_services = UsuarioServices::new();
    let result_busca = usuario_services.BuscarPorId(param_path.Id).await;
    match result_busca {
        Ok(usuario) => {
            return Ok(HttpResponse::Ok().json(usuario));
        }
        Err(err) => {
            return Ok(HttpResponse::NotFound().json("Usuario não encontrado"));
        }
    }
}
