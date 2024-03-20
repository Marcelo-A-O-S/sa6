use diesel::{table, MysqlConnection};
use crate::entities::Agendamento::{Agendamento,NovoAgendamento};
use super::TRepository::{TRepository};
use crate::schema::{agendamento, agendamento::dsl::*};
use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::*;
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
        let mut lista_agendamento: Vec<Agendamento> = Vec::new();
        match agendamento_table.load::<Agendamento>(&mut self.conn){
            Ok(results)=>{
                for agendamento_item in results {
                    let agendamento_ = Agendamento{
                        Id: agendamento_item.Id,
                        AcademiaId : agendamento_item.AcademiaId,
                        DataHoraId: agendamento_item.DataHoraId,
                        UsuarioId : agendamento_item.DataHoraId
                    };
                    lista_agendamento.push(agendamento_);
                    return Ok(lista_agendamento);
                }
            }
            Err(err)=>{
                return Err(()).expect("Erro ao listar dados");
            }
        }
        todo!()
    }

    async fn update(&mut self,entidade: Agendamento)->Result<(), diesel::result::Error> {
        let result = update(agendamento)
        .set((
            AcademiaId.eq(entidade.AcademiaId),
            DataHoraId.eq(entidade.DataHoraId),
            UsuarioId.eq(entidade.UsuarioId)
        ))
        .filter(Id.eq(entidade.Id))
        .execute(&mut self.conn);
    if result.is_ok() {
        Ok(())
    } else {
        Err(NotFound)
    }
    }

    async fn delete(&mut self,entidade: Agendamento)->Result<(), diesel::result::Error> {
        let result = delete(agendamento)
        .filter(Id.eq(entidade.Id))
        .execute(&mut self.conn);
    if result.is_ok() {
        Ok(())
    } else {
        Err(()).expect("Error ao deletar entidade")
    }
    }

    async fn deleteById(&mut self, _id:i32)->Result<(), diesel::result::Error> {
        let result = delete(agendamento).filter(Id.eq(_id)).execute(&mut self.conn);
        if result.is_ok() {
            Ok(())
        } else {
            Err(()).expect("Error ao deletar entidade")
        }
    }

    async fn findById(&mut self, _id: i32) ->Result<Agendamento, diesel::result::Error> {
        let result: Result<Agendamento, Error> = agendamento::table
            .filter(agendamento::Id.eq(_id))
            .first::<Agendamento>(&mut self.conn);
        match result {
            Ok(academia_result) => {
                return Ok(academia_result);
            }
            Err(e) => {
                return Err(()).expect("Erro ao buscar dado");
            }
        }
    }
}