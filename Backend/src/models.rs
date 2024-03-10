pub mod backend {
    use chrono::prelude::*;
    use std::collections::HashMap;
    use serde::{Deserialize, Serialize};

    #[derive(PartialEq, Debug, Hash, Eq, Clone, Deserialize, Serialize)]
    pub enum Mes {
        Janeiro,
        Fevereiro,
        Marco,
        Abril,
        Maio,
        Junho,
        Julho,
        Agosto,
        Setembro,
        Outubro,
        Novembro,
        Dezembro,
    }
    #[derive(PartialEq, Debug, Eq, Hash, Clone, Deserialize, Serialize)]
    pub struct DataHora {
        pub ano: u32,
        pub mes: Mes,
        pub dia: u32,
        pub hora: u32,
    }

    impl DataHora {
        pub fn new(ano: u32, mes: Mes, dia: u32, hora: u32) -> Self {
            let obj: DataHora = DataHora {
                ano,
                mes,
                dia,
                hora,
            };
            obj.validar_data();
            obj
        }

        pub fn validar_data(&self) {
            let ultimo_dia_possivel = match self.mes {
                Mes::Fevereiro => {
                    if self.ano % 4 == 0 && (self.ano % 100 != 0 || self.ano % 400 == 0) {
                        29
                    } else {
                        28
                    }
                }
                Mes::Janeiro
                | Mes::Marco
                | Mes::Maio
                | Mes::Julho
                | Mes::Agosto
                | Mes::Outubro
                | Mes::Dezembro => 31,
                Mes::Abril | Mes::Junho | Mes::Setembro | Mes::Novembro => 30,
            };

            if self.dia > ultimo_dia_possivel {
                panic!("esse dia não existe...")
            }

            match self.hora {
                0..=24 => {}
                _ => panic!("que horas?"),
            }
        }
        pub fn data_futura(&self) -> bool {
            let localtime: DateTime<Local> = Local::now();
            let ano = localtime.year() as u32;
            let mes = localtime.month();
            let dia = localtime.day();
            let hora = localtime.hour();
            let mesmarcado = match self.mes {
                Mes::Janeiro => 1,
                Mes::Fevereiro => 2,
                Mes::Marco => 3,
                Mes::Abril => 4,
                Mes::Maio => 5,
                Mes::Junho => 6,
                Mes::Julho => 7,
                Mes::Agosto => 8,
                Mes::Setembro => 9,
                Mes::Outubro => 10,
                Mes::Novembro => 11,
                Mes::Dezembro => 12,
            };

            if self.ano > ano {
                true
            } else if mesmarcado < mes || self.ano == ano {
                false
            } else if self.dia < dia || mesmarcado == mes || self.ano == ano {
                false
            } else if self.hora < hora || self.dia == dia {
                false
            } else {
                true
            }
        }
    }

    #[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
    pub struct Usuario {
        pub nome: String,
        pub cpf: String,
    }

    impl Usuario {
        pub fn new(nome: String, cpf: String) -> Self {
            user { nome, cpf }
        }
        pub fn validar_cpf(self) -> bool {
            self.cpf.chars().count() == 14
        }
    }

    #[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
    pub struct Agendamento {
        pub usuario: Usuario,
        pub horainicio: DataHora,
        pub horafim: DataHora,
    }

    impl Agendamento {
        pub fn new(usuario: Usuario, horainicio: DataHora, horafim: DataHora) -> Self {
            let temp = Agendamento {
                usuario,
                horainicio,
                horafim,
            };
            temp.validar_inicio_e_fim();
            temp
        }
        pub fn validar_inicio_e_fim(&self) {
            if !(self.horafim.data_futura() || self.horainicio.data_futura()) {
                panic!("você só pode agendar um horario no futuro")
            }
            if !(self.horafim.ano == self.horainicio.ano
                && self.horafim.mes == self.horainicio.mes
                && self.horafim.dia == self.horainicio.dia)
            {
                panic!("o horario de inicio e fim tem que ser no mesmo dia")
            } else if self.horafim.hora < self.horainicio.hora {
                panic!("o horario de saida não pode ser antes do de entrada")
            }
        }
    }

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
}
