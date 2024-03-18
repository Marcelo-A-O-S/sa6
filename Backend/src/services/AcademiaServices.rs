use super::TServices::TServices;
use crate::entities::Academia::{Academia};
use crate::repository::AcademiaRepository::AcademiaRepository;
use crate::repository::TRepository::TRepository;
use crate::utils::Errors::{ErrorProject};
pub struct AcademiaServices{
    repository:AcademiaRepository
}
impl AcademiaServices{
    pub fn new()-> AcademiaServices{
        AcademiaServices{
            repository: AcademiaRepository::new()
        }
    }
}
impl TServices<Academia> for AcademiaServices{
    async fn Salvar(&mut self, entidade :Academia)-> Result<(),ErrorProject> {
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

    async fn Deletar(&mut self, entidade: Academia) -> Result<(),ErrorProject> {
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

    async fn Atualizar(&mut self, entidade: Academia) -> Result<(),ErrorProject> {
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

    async fn Listar(&mut self) -> Result<Vec<Academia>, ErrorProject> {
        let listaAcademias = self.repository.listar().await;
        match listaAcademias {
            Ok(lista)=>{
                return Ok(lista);
            }
            Err(err)=>{
                Err(ErrorProject::NotFound(String::from("Retornou uma entidade vazia")))
            }
        }
    }
    
    async fn BuscarPorId(&mut self, _id: i32)->Result<Academia,ErrorProject> {
        let result =  self.repository.findById(_id).await;
        match result{
            Ok(academia)=>{
                return Ok(academia);
            }
            Err(err)=>{
                return Err(ErrorProject::NotFound(String::from("Retornou um valor vazio")));
            }
        }
    }

}
