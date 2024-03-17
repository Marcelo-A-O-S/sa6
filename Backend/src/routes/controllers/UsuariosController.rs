use actix_web::{get, post, delete, put, web, HttpResponse, Responder, Result, Error};
use crate::{entities::Usuarios::Usuario, services::{TServices::TServices, UsuarioServices::UsuarioServices}};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct UsuarioRequest{
     Id: i32,
     nome: String,
     CPF: String,
}


#[get("/Usuarios")]
async fn get_usuarios()->Result<HttpResponse>{
    let listaUsuarios = UsuarioServices::Listar().await;
    
    Ok(HttpResponse::Ok().json(listaUsuarios))
}
#[post("/CreateUsuario")]
async fn create_usuarios(usuarioJson : web::Json<UsuarioRequest>)->Result<HttpResponse, Error>{
    let usuarioBody = usuarioJson.into_inner();
    if usuarioBody.nome != "" && usuarioBody.CPF != ""{
        let mut usuario = Usuario::new(usuarioBody.Id, usuarioBody.nome, usuarioBody.CPF);
        if(usuarioBody.Id == 0){
            let usuarioServices = UsuarioServices::new();
            usuarioServices.Salvar(usuario).await;
            return Ok(HttpResponse::Ok().json("Usuario salvo com sucesso!"));
        }else{
            let usuarioServices = UsuarioServices::new();
            usuarioServices.Salvar(usuario).await;
            return Ok(HttpResponse::Ok().json("Usuario atualizado com sucesso!"));
        }
        
    }
    return Ok(HttpResponse::BadRequest().json("Campos inválidos"))
}
#[put("/UpdateUsuario")]
async fn update_usuario(usuarioJson : web::Json<UsuarioRequest>) -> Result<HttpResponse, Error>{
    let usuarioBody = usuarioJson.into_inner();
    if usuarioBody.nome != "" && usuarioBody.CPF != ""{
        let usuario = Usuario::new(usuarioBody.Id, usuarioBody.nome, usuarioBody.CPF);
        let usuarioServices = UsuarioServices::new();
        usuarioServices.Atualizar(usuario).await;
        return Ok(HttpResponse::Ok().json("Atualizado com sucesso"))
    }
    return Ok((HttpResponse::BadRequest().json("Campos inválidos")))
}
#[delete("/DeleteUsuario")]
async fn delete_usuario(usuarioJson: web::Json<UsuarioRequest>) -> Result<HttpResponse, Error>{
    let usuarioBody = usuarioJson.into_inner();
    if usuarioBody.nome != "" && usuarioBody.CPF != ""{
        let usuario = Usuario::new(usuarioBody.Id, usuarioBody.nome, usuarioBody.CPF);
        let usuarioServices = UsuarioServices::new();
        usuarioServices.Deletar(usuario).await;
        return Ok(HttpResponse::Ok().json("Atualizado com sucesso"))
    }
    return Ok((HttpResponse::BadRequest().json("Campos inválidos")))
}
#[post("/DeleteUsuario")]
async fn post_delete_usuario(usuarioJson: web::Json<UsuarioRequest>) -> Result<HttpResponse, Error>{
    let usuarioBody = usuarioJson.into_inner();
    if usuarioBody.nome != "" && usuarioBody.CPF != ""{
        let usuario = Usuario::new(usuarioBody.Id, usuarioBody.nome, usuarioBody.CPF);
        let usuarioServices = UsuarioServices::new();
        usuarioServices.Deletar(usuario).await;
        return Ok(HttpResponse::Ok().json("Atualizado com sucesso"))
    }
    return Ok((HttpResponse::BadRequest().json("Campos inválidos")))
}
