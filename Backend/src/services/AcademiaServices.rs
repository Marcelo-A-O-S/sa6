use super::TServices::TServices;
use crate::entities::Usuarios::Usuario;
use crate::entities::{
    Academia::Academia,
    Agendamento::Agendamento,
    DataHora::DataHora
};
use crate::repository::{
    AcademiaRepository::AcademiaRepository,
    AgendamentoRepository::AgendamentoRepository,
    DataHoraRepository::DataHoraRepository,
    UsuariosRepository::UsuarioRepository
};
use crate::repository::TRepository::TRepository;
use crate::utils::Errors::{ErrorProject};
pub struct AcademiaServices{
    repository_academia:AcademiaRepository,
    repository_agendamento: AgendamentoRepository,
    repository_dh: DataHoraRepository,
    repository_user: UsuarioRepository
}
impl AcademiaServices{
    pub fn new()-> AcademiaServices{
        AcademiaServices{
            repository_academia: AcademiaRepository::new(),
            repository_agendamento: AgendamentoRepository::new(),
            repository_dh: DataHoraRepository::new(),
            repository_user: UsuarioRepository::new()
        }
    }
    pub async fn deletar_agendamento_by_id(&mut self, agendamento_id:i32)->Result<(), ErrorProject>{
        let mut result_busca = self.repository_agendamento.findById(agendamento_id).await;
        match result_busca {
            Ok(agendamento)=>{
                let result_delete =  self.repository_agendamento.deleteById(agendamento.Id).await;
                match result_delete{
                    Ok(())=>{
                        return Ok(());
                    }
                    Err(err)=>{
                        return Err(ErrorProject::ErrorUpdateDB(err))
                    }
                }
            }
            Err(err)=>{
                return Err(ErrorProject::NotFound(err))
            }
        }
    }
    pub async fn atualizar_agendamento(&mut self, agendamento_id:i32, entidade_dh:DataHora, academia_id: i32, usuario_id:i32) -> Result<(), ErrorProject>{
        let update_dh = self.repository_dh.update(entidade_dh).await;
        match update_dh{
            Ok(())=>{

                let mut agendamento = self.repository_agendamento.findById(agendamento_id).await.unwrap();
                agendamento.AcademiaId = academia_id;
                agendamento.DataHoraId = entidade_dh.Id;
                agendamento.UsuarioId = usuario_id;
                self.repository_agendamento.update(agendamento).await;
                return Ok(());
            }
            Err(err)=>{
                return Err(ErrorProject::ErrorUpdateDB(err))
            }
        }
    }
    pub async fn salvar_agendamento(&mut self, _academia_id: i32, usuario_id : i32, entidade_dh: DataHora ) -> Result<(), ErrorProject>{
        let save_dh = self.repository_dh.salvar(entidade_dh).await;
        if save_dh.is_ok(){
            let result_find = self.repository_dh.findLastId().await;
            match result_find{
                Ok(data)=>{
                    let agendamento = Agendamento{
                        AcademiaId: _academia_id,
                        DataHoraId: data.Id,
                        UsuarioId: usuario_id,
                        Id: 0
                    };
                    let save_agendamento = self.repository_agendamento.salvar(agendamento).await;
                    if save_agendamento.is_ok(){
                        return Ok(());
                    }else{
                        return Err(ErrorProject::ErrorUpdateDB(String::from("Erro ao salvar entidade")));
                    }
                }
                Err(err)=>{
                    return Err(ErrorProject::ErrorUpdateDB(err));
                }
            }
        }else{
            return Err(ErrorProject::ErrorUpdateDB(String::from("Erro ao salvar data")));
        }
       
    }
    pub async fn buscar_agendamentos_por_academiaid(&mut self, academia_id: i32)-> Result<Vec<Agendamento>, ErrorProject>{
        let result_busca = self.repository_agendamento.get_by_academia_id(academia_id).await;
        match result_busca {
            Ok(agendamentos)=>{
               return Ok(agendamentos);
            }
            Err(err)=>{
                return Err(ErrorProject::ErrorUpdateDB(err));
            }
        }
    }
    pub async fn buscar_dh_por_id(&mut self, dh_id: i32)-> Result<DataHora, ErrorProject>{
        let result_busca = self.repository_dh.findById(dh_id).await;
        match result_busca{
            Ok(dh)=>{
                return Ok(dh);
            }
            Err(err)=>{
                return Err(ErrorProject::ErrorUpdateDB(err))
            }
        }
    }
    pub async fn buscar_agendamento_por_Id(&mut self, agendamento_id: i32)-> Result<Agendamento, ErrorProject>{
        let mut result_busca = self.repository_agendamento.findById(agendamento_id).await;
        match result_busca{
            Ok(agendamento) =>{
                return Ok(agendamento);
            }
            Err(err)=>{
                return Err(ErrorProject::ErrorUpdateDB(err));
            }
        }
    }
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
