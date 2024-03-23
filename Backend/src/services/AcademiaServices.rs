use super::TServices::TServices;
use crate::entities::{
    Academia::Academia,
    Agendamento::Agendamento,
    DataHora::DataHora
};
use crate::repository::{
    AcademiaRepository::AcademiaRepository,
    AgendamentoRepository::AgendamentoRepository,
    DataHoraRepository::DataHoraRepository
};
use crate::repository::TRepository::TRepository;
use crate::utils::Errors::{ErrorProject};
pub struct AcademiaServices{
    repository_academia:AcademiaRepository,
    repository_agendamento: AgendamentoRepository,
    repository_dh: DataHoraRepository
}
impl AcademiaServices{
    pub fn new()-> AcademiaServices{
        AcademiaServices{
            repository_academia: AcademiaRepository::new(),
            repository_agendamento: AgendamentoRepository::new(),
            repository_dh: DataHoraRepository::new()
        }
    }
   /*  pub async fn SalvarAgendamento(&mut self, _academiaId: i32, entidade_agendamento : Agendamento, entidade_dh: DataHora ) -> Result<(), ErrorProject>{
        let save_dh = self.repository_dh.salvar(entidade_dh).await;
        match save_dh{
             Ok(())=>{
                let result_dh = self.repository_dh.findLastId().await;
                match result_dh {
                    Ok(datahora_return)=>{
                        
                    }
                    Err(err)=>{

                    }
                }
             }
             Err(err)=>{
                return Err(ErrorProject::ErrorGeneric(String::from("Cu doce")))
             }
        }
       
    } */
}
impl TServices<Academia> for AcademiaServices{
    async fn Salvar(&mut self, entidade :Academia)-> Result<(),ErrorProject> {
        if entidade.Id == 0{
            let resultSave = self.repository_academia.salvar(entidade).await;
            match resultSave {
                Ok(())=>{
                     return Ok(())
                }
                Err(err)=>{
                     return Err(ErrorProject::ErrorUpdateDB(String::from("Erro ao salvar no banco de dados")))
                }
            }
         }else{
             let resultUpdate = self.repository_academia.update(entidade).await;
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
        let result = self.repository_academia.delete(entidade).await;
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
        let result = self.repository_academia.update(entidade).await;
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
        let listaAcademias = self.repository_academia.listar().await;
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
        let result =  self.repository_academia.findById(_id).await;
        match result{
            Ok(academia)=>{
                return Ok(academia);
            }
            Err(err)=>{
                return Err(ErrorProject::NotFound(String::from("Retornou um valor vazio")));
            }
        }
    }
    
    async fn DeletarPorId(&mut self,_id: i32) ->Result<(),ErrorProject> {
        let result = self.repository_academia.deleteById(_id).await;
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
