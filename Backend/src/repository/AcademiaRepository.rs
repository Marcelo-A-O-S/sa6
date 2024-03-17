use super::TRepository::TRepository;
use crate::connection::estabilishConnection;
use crate::entities::Academia::{Academia, NovaAcademia};
use crate::schema::{academia, academia::dsl::*};

use diesel::prelude::*;
use diesel::*;

use diesel::RunQueryDsl;
pub struct AcademiaRepository {
    conn: MysqlConnection

}
impl AcademiaRepository{
    pub fn new()->AcademiaRepository{
        AcademiaRepository{
            conn: estabilishConnection()
        }
    }
}
impl TRepository<Academia> for AcademiaRepository {
    async fn salvar(&mut self,entidade: Academia) {
        let nova_academia = NovaAcademia {
            NomeComercial: entidade.NomeComercial,
            CapacidadeUsuarios: entidade.CapacidadeUsuarios,
            Endereco: entidade.Endereco,
            HorarioAbertura: entidade.HorarioAbertura,
            HorarioFechamento: entidade.HorarioFechamento,
        };
        diesel::insert_into(academia::table)
            .values(&nova_academia)
            .execute(&mut self.conn)
            .expect("Erro ao inserir dados");
    }

    async fn listar(&mut self) -> Vec<Academia> {
        
        let mut listaAcademias: Vec<Academia> = Vec::new();
        match academia.load::<Academia>(&mut self.conn) {
            Ok((results)) => {
                for academiaData in results {
                    let academia_entities = Academia::new(
                        academiaData.Id,
                        academiaData.NomeComercial,
                        academiaData.HorarioAbertura,
                        academiaData.HorarioFechamento,
                        academiaData.CapacidadeUsuarios,
                        academiaData.Endereco,
                    );
                    listaAcademias.push(academia_entities)
                }
                listaAcademias
            }
            Err(err) => {
                println!("Ocorreu o seguinte erro: {}!", err);
                listaAcademias
            }
        }
    }

    async fn update(&mut self,entidade: Academia) {
        
        update(academia)
            .set((
                Endereco.eq(entidade.Endereco),
                HorarioFechamento.eq(entidade.HorarioFechamento),
                HorarioAbertura.eq(entidade.HorarioAbertura),
                CapacidadeUsuarios.eq(entidade.CapacidadeUsuarios),
            ))
            .filter(Id.eq(entidade.Id))
            .execute(&mut self.conn).expect("Houve um erro ao atualizar entidade");
        todo!()
    }
    
    async fn delete(&mut self,entidade: Academia) {
        delete(academia)
        .filter(Id.eq(entidade.Id))
        .execute(&mut self.conn).expect("A entidade não foi deletada com sucesso");

    }
    
    async fn findById(&mut self, _id: i32) -> Academia {
        todo!()
    }
    
    async fn deleteById(&mut self, _id:i32) {
        delete(academia)
        .filter(Id.eq(_id))
        .execute(&mut self.conn).expect("A entidade não foi deletada com sucesso");
    }
}
