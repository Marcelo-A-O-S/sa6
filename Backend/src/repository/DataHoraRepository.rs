use diesel::MysqlConnection;
use crate::entities::DataHora::{DataHora, NovaDataHora};
use crate::schema::{datahora, datahora::dsl::*};
use super::TRepository::TRepository;
use crate::connection::estabilishConnection;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::RunQueryDsl;
use diesel::*;

pub struct DataHoraRepository{
    conn: MysqlConnection
}
impl DataHoraRepository{
    pub fn new()->DataHoraRepository{
        DataHoraRepository{
            conn: estabilishConnection()
        }
    }
}
impl TRepository<DataHora> for DataHoraRepository {
    async fn salvar(&mut self,entidade: DataHora)->Result<(), String> {
        let novadatahora = NovaDataHora{
            Ano : entidade.Ano,
            Dia: entidade.Dia,
            Hora: entidade.Hora,
            Mes: entidade.Mes
        };
        let result = diesel::insert_into(datahora::table)
            .values(&novadatahora)
            .execute(&mut self.conn);
        if result.is_ok(){
            return Ok(())
        }else{
            return Err(String::from("Error ao salvar entidade"));
        }
    }

    async fn listar(&mut self) -> Result<Vec<DataHora>, String> {
        let mut datahora_table = datahora::table;
        let mut lista_datahora: Vec<DataHora> = Vec::new();
        match datahora_table.load::<DataHora>(&mut self.conn){
            Ok(results)=>{
                for datahora_item in results {
                    let datahora_ = DataHora{
                        Id: datahora_item.Id,
                        Ano: datahora_item.Ano,
                        Dia: datahora_item.Dia,
                        Hora: datahora_item.Hora,
                        Mes: datahora_item.Mes,
                    };
                    lista_datahora.push(datahora_);
                }
                return Ok(lista_datahora);
            }
            Err(err)=>{
                return Err(String::from("Houve um problema ao puxar a lista de entidade"));
            }
        }
    }

    async fn update(&mut self,entidade: DataHora)->Result<(), String> {
        let result = update(datahora)
        .set((
            Ano.eq(entidade.Ano),
            Mes.eq(entidade.Mes),
            Dia.eq(entidade.Dia),
            Hora.eq(entidade.Hora)
        ))
        .filter(Id.eq(entidade.Id))
        .execute(&mut self.conn);
        if result.is_ok() {
            Ok(())
        } else {
            Err(String::from("Não foi possivel atualizar a entidade"))
        }
    }

    async fn delete(&mut self,entidade: DataHora)->Result<(), String> {
        let result = delete(datahora)
        .filter(Id.eq(entidade.Id))
        .execute(&mut self.conn);
        if result.is_ok() {
            Ok(())
        } else {
            Err(String::from("Error ao deletar entidade"))
        }
    }

    async fn deleteById(&mut self, _id:i32)->Result<(), String> {
        let result = delete(datahora).filter(Id.eq(_id)).execute(&mut self.conn);
        if result.is_ok() {
            Ok(())
        } else {
            Err(String::from("Error ao deletar entidade"))
        }
    }

    async fn findById(&mut self, _id: i32) ->Result<DataHora, String> {
        let result: Result<DataHora, Error> = datahora::table
            .filter(datahora::Id.eq(_id))
            .first::<DataHora>(&mut self.conn);
            match result {
                Ok(datahora_result) => {
                    return Ok(datahora_result);
                }
                Err(e) => {
                    return Err(String::from("Erro ao buscar dado"));
                }
            }
    }
    
    async fn findLastId(&mut self) ->Result<DataHora,String> {
        let result = datahora
        .select(datahora::all_columns)
        .order(Id.desc())
        .limit(1)
        .first::<DataHora>(&mut self.conn);
        match result {
            Ok(data_hora)=>{
                return Ok(data_hora)
            }
            Err(err)=>{
                return Err(String::from("Erro nesse baguio"));
            }
        }
    }
}