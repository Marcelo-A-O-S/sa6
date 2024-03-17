//use std::collections::HashMap;
use diesel::*;
use serde::{Serialize};
use chrono::{NaiveTime};
use crate::schema::{academia};
use super::Agendamento::Agendamento;
#[derive(Queryable, PartialEq, Serialize, Debug, Clone)]
    pub struct Academia {
        pub Id: i32,
        pub NomeComercial: String,
        pub Endereco: String,
        pub HorarioAbertura: NaiveTime,
        pub HorarioFechamento: NaiveTime,
        pub CapacidadeUsuarios: i32,
       // pub horariosagendados: HashMap<DataHora, Vec<Agendamento>>,
    }
    //the trait bound `(i32, std::string::String, std::string::String, NaiveTime, NaiveTime, i32)
#[derive(Insertable)]
#[diesel(table_name = academia)]
    pub struct NovaAcademia{
        pub NomeComercial: String,
        pub Endereco: String,
        pub HorarioAbertura: NaiveTime,
        pub HorarioFechamento: NaiveTime,
        pub CapacidadeUsuarios: i32
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
            //self.validar_horario(&agendamento.horainicio.hora, &agendamento.horafim.hora);
/*             if self.horariosagendados.contains_key(&agendamento.horainicio) {
                let temp: &mut Vec<Agendamento> = self.horariosagendados.get_mut(&agendamento.horainicio).unwrap();
                if temp.len() < self.CapacidadeUsuarios as usize {
                    temp.push(agendamento);
                } else {
                    panic!("lotado")
                }
            } else {
                let mut temp = Vec::new();
                temp.push(agendamento.clone());
                self.horariosagendados.insert(agendamento.horainicio, temp);
            } */
        }
    }
