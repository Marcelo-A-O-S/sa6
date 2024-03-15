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
    }