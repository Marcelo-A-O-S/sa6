

use diesel::result;

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
    pub async fn verificarExistenciaCpf(&mut self, mut _cpf: &String) -> Result<bool, bool> {
        let result = self.repository.findByCPF(_cpf).await;
        if result.is_ok() == true {
            return Ok(true)
        }else{
            return Ok(false)
        }
    }
    pub async fn BuscarPorCpf(&mut self, mut _cpf: &String) -> Result<Usuario, ErrorProject> {
        let result = self.repository.findByCPF(_cpf).await;
        match result{
            Ok(usuario)=>{
                return Ok(usuario);
            }
            Err(err)=>{
                return Err(ErrorProject::NotFound(String::from("Usuário não encontrado")));
            }
        }
    }
    pub async fn ValidarSeCpfPertenceUsuario(&mut self, mut _cpf: &String, usuario_param: &Usuario) -> Result<bool, ErrorProject> {
        let result =  self.repository.findByCPF(_cpf).await;
        match result {
            Ok(usuario)=>{
                if usuario.CPF == usuario_param.CPF && usuario.nome == usuario_param.nome && usuario.Id == usuario_param.Id{
                    return Ok(true);
                }else{
                    return Ok(false);
                }
            }
            Err(err)=>{
                return Err(ErrorProject::NotFound(String::from("Erro ao buscar dados")));
            }
        }
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
    
    async fn DeletarPorId(&mut self,_id: i32) ->Result<(),ErrorProject> {
        let result = self.repository.deleteById(_id).await;
        match result{
            Ok(())=>{
                return Ok(());
            }
            Err(err)=>{
                return Err(ErrorProject::NotFound(String::from("Erro ao atualizar no banco de dados")));
            }
        }
    }
}