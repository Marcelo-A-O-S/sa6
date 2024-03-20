use diesel::{table, MysqlConnection};
use crate::entities::Agendamento::{Agendamento,NovoAgendamento};
use super::TRepository::{TRepository};
use crate::schema::{agendamento, agendamento::dsl::*};
use diesel::RunQueryDsl;
pub struct AgendamentoRepository{
    conn : MysqlConnection
}
impl TRepository<Agendamento> for AgendamentoRepository{
    async fn salvar(&mut self,entidade: Agendamento)->Result<(), diesel::result::Error> {
        let novagendamento = NovoAgendamento{
            AcademiaId : entidade.AcademiaId,
            DataHoraId : entidade.DataHoraId,
            UsuarioId : entidade.UsuarioId
        };
        let result = diesel::insert_into(agendamento::table)
            .values(&novagendamento)
            .execute(&mut self.conn);
        if result.is_ok(){
            return Ok(())
        }else{
            return Err(()).expect("Error ao salvar entidade")
        }
    }

    async fn listar(&mut self) -> Result<Vec<Agendamento>, diesel::result::Error> {
        let mut agendamento_table = agendamento::table;
        let lista_agendamento: Vec<Agendamento> = Vec::new();
        match agendamento_table.load::<Agendamento>(&mut self.conn){
            Ok(results)=>{
            
            }
            Err(err)=>{

            }
        }
        todo!()
    }

    async fn update(&mut self,entidade: Agendamento)->Result<(), diesel::result::Error> {
        todo!()
    }

    async fn delete(&mut self,entidade: Agendamento)->Result<(), diesel::result::Error> {
        todo!()
    }

    async fn deleteById(&mut self, _id:i32)->Result<(), diesel::result::Error> {
        todo!()
    }

    async fn findById(&mut self, _id: i32) ->Result<Agendamento, diesel::result::Error> {
        todo!()
    }
}