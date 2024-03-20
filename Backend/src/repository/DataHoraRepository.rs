use diesel::MysqlConnection;

use crate::entities::DataHora::DataHora;

use super::TRepository::TRepository;

pub struct DataHoraRepository{
    conn: MysqlConnection
}
impl TRepository<DataHora> for DataHoraRepository {
    async fn salvar(&mut self,entidade: DataHora)->Result<(), diesel::result::Error> {
        todo!()
    }

    async fn listar(&mut self) -> Result<Vec<DataHora>, diesel::result::Error> {
        todo!()
    }

    async fn update(&mut self,entidade: DataHora)->Result<(), diesel::result::Error> {
        todo!()
    }

    async fn delete(&mut self,entidade: DataHora)->Result<(), diesel::result::Error> {
        todo!()
    }

    async fn deleteById(&mut self, _id:i32)->Result<(), diesel::result::Error> {
        todo!()
    }

    async fn findById(&mut self, _id: i32) ->Result<DataHora, diesel::result::Error> {
        todo!()
    }
}