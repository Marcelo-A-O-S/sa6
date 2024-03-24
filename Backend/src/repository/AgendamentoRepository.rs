use diesel::{table, MysqlConnection};
use crate::entities::Agendamento::{Agendamento,NovoAgendamento};
use super::TRepository::{TRepository};
use crate::connection::estabilishConnection;
use crate::schema::{agendamento, agendamento::dsl::*};
use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::*;
pub struct AgendamentoRepository{
    conn : MysqlConnection
}
impl AgendamentoRepository{
    pub fn new()-> AgendamentoRepository{
        AgendamentoRepository{
            conn: estabilishConnection()
        }
    }
    pub async fn get_by_academia_id(&mut self, academia_id: i32)->Result<Vec<Agendamento>, String>{
        let result: Result<Vec<Agendamento>, Error> = agendamento::table.select(agendamento::all_columns)
        .filter(agendamento::AcademiaId.eq(academia_id))
        .get_results(&mut self.conn);
        match result {
            Ok(agendamentos)=>{
                return Ok(agendamentos);
            }
            Err(err)=>{
                return Err(String::from("Erro ao buscar dados"));
            }
        }
    }
}

impl TRepository<Agendamento> for AgendamentoRepository{
    async fn salvar(&mut self,entidade: Agendamento)->Result<(), String> {
        let novagendamento = NovoAgendamento{
            AcademiaId : entidade.AcademiaId,
            DataHoraId : entidade.DataHoraId,
            UsuarioId : entidade.UsuarioId
        };
        let mut result = diesel::insert_into(agendamento::table)
            .values(&novagendamento)
            .execute(&mut self.conn);
        match result {
            Ok(rows)=>{
                return Ok(());
            }
            Err(err)=>{
                println!("{:?}", err);
                return Err(String::from("Erro ao salvar dados"));
            }
        }
    }

    async fn listar(&mut self) -> Result<Vec<Agendamento>, String> {
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
                }
                return Ok(lista_agendamento);
            }
            Err(err)=>{
                return Err(String::from("Erro ao listar dados"));
            }
        }
    }

    async fn update(&mut self,entidade: Agendamento)->Result<(), String> {
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
            Err(String::from("Erro ao atualizar dado"))
        }
    }

    async fn delete(&mut self,entidade: Agendamento)->Result<(), String> {
        let result = delete(agendamento)
        .filter(Id.eq(entidade.Id))
        .execute(&mut self.conn);
        if result.is_ok() {
            Ok(())
        } else {
            Err(String::from("Error ao deletar entidade"))
        }
    }

    async fn deleteById(&mut self, _id:i32)->Result<(), String> {
        let result = delete(agendamento).filter(Id.eq(_id)).execute(&mut self.conn);
        if result.is_ok() {
            Ok(())
        } else {
            Err(String::from("Error ao deletar entidade"))
        }
    }

    async fn findById(&mut self, _id: i32) ->Result<Agendamento, String> {
        let result: Result<Agendamento, Error> = agendamento::table
            .filter(agendamento::Id.eq(_id))
            .first::<Agendamento>(&mut self.conn);
            match result {
                Ok(academia_result) => {
                    return Ok(academia_result);
                }
                Err(e) => {
                    return Err(String::from("Erro ao buscar dado"));
                }
            }
    }
    
    async fn findLastId(&mut self) ->Result<Agendamento,String> {
        let result = agendamento
        .select(agendamento::all_columns)
        .order(Id.desc())
        .limit(1)
        .first::<Agendamento>(&mut self.conn);
        match result {
            Ok(agendamento_body)=>{
                return Ok(agendamento_body)
            }
            Err(err)=>{
                return Err(String::from("Erro nesse baguio"))
            }
        }
    }
}