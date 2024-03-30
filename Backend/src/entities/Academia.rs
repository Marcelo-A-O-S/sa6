use super::Agendamento::Agendamento;
use crate::schema::academia;
use chrono::NaiveTime;
use diesel::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Identifiable, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[diesel(table_name = academia)]
#[diesel(primary_key(Id))]
pub struct Academia {
    pub Id: i32,
    pub CapacidadeUsuarios: i32,
    pub NomeComercial: String,
    pub Endereco: String,
    pub HorarioAbertura: NaiveTime,
    pub HorarioFechamento: NaiveTime,
}
#[derive(Insertable)]
#[diesel(table_name = academia)]
pub struct NovaAcademia {
    pub NomeComercial: String,
    pub Endereco: String,
    pub HorarioAbertura: NaiveTime,
    pub HorarioFechamento: NaiveTime,
    pub CapacidadeUsuarios: i32,
}
impl Academia {
    pub fn new(
        Id: i32,
        NomeComercial: String,
        HorarioAbertura: NaiveTime,
        HorarioFechamento: NaiveTime,
        CapacidadeUsuarios: i32,
        Endereco: String,
        //horariosagendados: HashMap<DataHora, Vec<Agendamento>>,
    ) -> Self {
        Academia {
            Id,
            NomeComercial,
            HorarioAbertura,
            HorarioFechamento,
            CapacidadeUsuarios,
            Endereco,
            //horariosagendados,
        }
    }
    pub fn validar_horario(&self, horainicio: &NaiveTime, horafim: &NaiveTime) {
        if *horainicio < self.HorarioAbertura || *horafim > self.HorarioFechamento {
            panic!("horario fora do funcionamento")
        }
    }
    pub fn agendar(&mut self, agendamento: Agendamento) {
        /* self.validar_horario(&agendamento.HorarioAbertura, &agendamento.HorarioFechamento);
               if self.horariosagendados.contains_key(&agendamento.HorarioAbertura) {
                let temp: &mut Vec<Agendamento> = self.horariosagendados.get_mut(&agendamento.HorarioAbertura).unwrap();
                if temp.len() < self.CapacidadeUsuarios as usize {
                    temp.push(agendamento);
                } else {
                    panic!("lotado")
                }
            } else {
                let mut temp = Vec::new();
                temp.push(agendamento.clone());
                self.horariosagendados.insert(agendamento.HorarioAbertura, temp);
            }
        } */
    }
}
