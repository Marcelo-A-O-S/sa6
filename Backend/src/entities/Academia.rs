use std::collections::HashMap;
use super::DataHora::DataHora;
use super::Agendamento::Agendamento;
#[derive(Debug)]
    pub struct Academia {
        pub nome: String,
        pub horario_abertura: u32,
        pub horario_fechamento: u32,
        pub capacidade: u32,
        pub horariosagendados: HashMap<DataHora, Vec<Agendamento>>,
    }

    impl Academia {
        pub fn new(
            nome: String,
            horario_abertura: u32,
            horario_fechamento: u32,
            capacidade: u32,
            horariosagendados: HashMap<DataHora, Vec<Agendamento>>,
        ) -> Self {
            Academia {
                nome,
                horario_abertura,
                horario_fechamento,
                capacidade,
                horariosagendados,
            }
        }
        pub fn validar_horario(&self, horainicio: &DataHora, horafim: &DataHora) {
            if horainicio.hora < self.horario_abertura || horafim.hora > self.horario_fechamento {
                panic!("horario fora do funcionamento")
            }
        }
        pub fn agendar(&mut self, agendamento: Agendamento) {
            self.validar_horario(&agendamento.horainicio, &agendamento.horafim);
            if self.horariosagendados.contains_key(&agendamento.horainicio) {
                let temp: &mut Vec<Agendamento> = self.horariosagendados.get_mut(&agendamento.horainicio).unwrap();
                if temp.len() < self.capacidade as usize {
                    temp.push(agendamento);
                } else {
                    panic!("lotado")
                }
            } else {
                let mut temp = Vec::new();
                temp.push(agendamento.clone());
                self.horariosagendados.insert(agendamento.horainicio, temp);
            }
        }
    }
