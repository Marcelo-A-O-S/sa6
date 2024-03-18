use std::io::{empty, Empty};

use crate::entities::Usuarios::Usuario;
use crate::repository::{TRepository::TRepository,UsuariosRepository::UsuarioRepository};
use super::TServices::TServices;
use crate::utils::Errors::{ErrorProject};
pub struct UsuarioServices{
    repository: UsuarioRepository
}
impl UsuarioServices {
    pub fn new() -> UsuarioServices {
        return UsuarioServices { repository: UsuarioRepository::new() }
    }
}
impl TServices<Usuario> for UsuarioServices{
    
    async fn Salvar(&mut self, entidade :Usuario)-> Result<(),ErrorProject> {
        
        if entidade.Id == 0{
           let resultSave = self.repository.salvar(entidade).await;
           match resultSave {
               Ok(())=>{
                    return Ok(())
               }
               Err(err)=>{
                    return Err(ErrorProject::ErrorUpdateDB(String::from("Erro ao salvar no banco de dados")))
               }
           }
            
        }else{
            let resultUpdate = self.repository.update(entidade).await;
            match resultUpdate {
                Ok(())=>{
                     return Ok(())
                }
                Err(err)=>{
                     return Err(ErrorProject::ErrorUpdateDB(String::from("Erro ao atualizar no banco de dados")))
                }
            }
             
        }
        
    }

    async fn Deletar(&mut self, entidade: Usuario) -> Result<(),ErrorProject>{
        let result = self.repository.delete(entidade).await;
        match result{
            Ok(())=>{
                return Ok(());
            }
            Err(err)=>{
                return Err(ErrorProject::NotFound(String::from("Erro ao deletar no banco de dados")));
            }
        }
    }

    async fn Atualizar(&mut self, entidade: Usuario) -> Result<(),ErrorProject> {
        let result = self.repository.update(entidade).await;
        match result{
            Ok(())=>{
                return Ok(());
            }
            Err(err)=>{
                return Err(ErrorProject::NotFound(String::from("Erro ao atualizar no banco de dados")));
            }
        }
    }

    async fn Listar(&mut self) -> Result<Vec<Usuario>, ErrorProject> {
        let listaUsuarios = self.repository.listar().await;
        match listaUsuarios {
            Ok(lista)=>{
                return Ok(lista);
            }
            Err(err)=>{
                return Err(ErrorProject::NotFound(String::from("O objeto retornou vazio")));
            }
        }
    }
    
    async fn BuscarPorId(&mut self, _id: i32)->Result<Usuario,ErrorProject>{
        let result =  self.repository.findById(_id).await;
        match result{
            Ok(usuario)=>{
                return Ok(usuario);
            }
            Err(err)=>{
                return Err(ErrorProject::NotFound(String::from("Retornou um valor vazio")));
            }
        }
    }
}