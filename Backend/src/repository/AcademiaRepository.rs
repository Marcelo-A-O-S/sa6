use super::TRepository::TRepository;
use crate::connection::estabilishConnection;
use crate::entities::Academia::{Academia, NovaAcademia};
use crate::schema::{academia, academia::dsl::*};

use diesel::prelude::*;
use diesel::*;

use diesel::RunQueryDsl;
struct AcademiaRepository {}
impl TRepository<Academia> for AcademiaRepository {
    async fn salvar(entidade: Academia) {
        let nova_academia = NovaAcademia {
            NomeComercial: entidade.NomeComercial,
            CapacidadeUsuarios: entidade.CapacidadeUsuarios,
            Endereco: entidade.Endereco,
            HorarioAbertura: entidade.HorarioAbertura,
            HorarioFechamento: entidade.HorarioFechamento,
        };
        let conn = &mut estabilishConnection();
        diesel::insert_into(academia::table)
            .values(&nova_academia)
            .execute(conn)
            .expect("Erro ao inserir dados");
    }

    async fn listar() -> Vec<Academia> {
        let conn = &mut estabilishConnection();
        let mut listaAcademias: Vec<Academia> = Vec::new();
        match academia.load::<Academia>(conn) {
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

    async fn update(entidade: Academia) {
        let conn = &mut estabilishConnection();
        update(academia::table.filter(Id.eq(entidade.Id)))
        .set((
            Endereco.eq(entidade.Endereco),
            HorarioFechamento.eq(entidade.HorarioFechamento),
            HorarioAbertura.eq(entidade.HorarioAbertura),
            CapacidadeUsuarios.eq(entidade.CapacidadeUsuarios)
        ))
        .execute(conn);
    }
}
