

use chrono::Local;
use chrono::DateTime;
use chrono::Timelike;
use chrono::Datelike;
use chrono::NaiveTime;
use crate::entities::enums::EMes::Mes;

#[derive(PartialEq, Debug, Eq, Hash, Clone)]
    pub struct DataHora {
        pub ano: u32,
        pub mes: Mes,
        pub dia: u32,
        pub hora: NaiveTime,
    }

    impl DataHora {
        pub fn new(ano: u32, mes: Mes, dia: u32, hora: NaiveTime) -> Self {
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


        }
        pub fn data_futura(&self) -> bool {
            let localtime: DateTime<Local> = Local::now();
            let ano = localtime.year() as u32;
            let mes = localtime.month();
            let dia = localtime.day();
            let hora = NaiveTime::from_hms_opt(localtime.hour(),localtime.minute(),localtime.second()).unwrap();
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