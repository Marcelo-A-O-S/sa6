
use diesel::*;
use crate::schema::{datahora};
use chrono::Local;
use chrono::DateTime;
use chrono::Timelike;
use chrono::Datelike;
use chrono::NaiveTime;
#[derive(Debug, Identifiable, Queryable,  Clone)]
#[diesel(table_name = datahora)]
#[diesel(primary_key(Id))]
    pub struct DataHora {
        pub Id: i32,
        pub Ano: i32,
        pub Mes: i32,
        pub Dia: i32,
        pub Hora: NaiveTime,
    }
#[derive(Insertable)]
#[diesel(table_name = datahora)]
    pub struct NovaDataHora{
        pub Ano: i32,
        pub Mes: i32,
        pub Dia: i32,
        pub Hora: NaiveTime,
    }

    impl DataHora {
        pub fn new(id:i32,ano: i32, mes: i32, dia: i32, hora: NaiveTime) -> Self {
            let obj: DataHora = DataHora {
                Id: id,
                Ano: ano,
                Mes: mes,
                Dia: dia,
                Hora: hora,
            };
            obj.validar_data();
            obj
        }

        pub fn validar_data(&self) {
            let ultimo_dia_possivel = match self.Mes {
                //Mes::Fevereiro
                2 => {
                    if self.Ano % 4 == 0 && (self.Ano % 100 != 0 || self.Ano % 400 == 0) {
                        29
                    } else {
                        28
                    }
                }
                1
                |3
                |5
                |7
                |8
                |10
                |12 => 31,
                4
                |6
                |9
                |11 => 30,
                _ => panic!("Mês inválido")
                /* Mes::Janeiro
                | Mes::Marco
                | Mes::Maio
                | Mes::Julho
                | Mes::Agosto
                | Mes::Outubro
                | Mes::Dezembro => 31,
                Mes::Abril | Mes::Junho | Mes::Setembro | Mes::Novembro => 30, */
            };

            if self.Dia > ultimo_dia_possivel {
                panic!("esse dia não existe...")
            }


        }
        pub fn data_futura(&self) -> bool {
            let localtime: DateTime<Local> = Local::now();
            let ano = localtime.year() as i32;
            let mes = localtime.month() as i32;
            let dia = localtime.day() as i32;
            let hora = NaiveTime::from_hms_opt(localtime.hour(),localtime.minute(),localtime.second()).unwrap();
            let mesmarcado =  self.Mes;

            if self.Ano > ano {
                true
            } else if mesmarcado < mes || self.Ano == ano {
                false
            } else if self.Dia < dia || mesmarcado == mes || self.Ano == ano {
                false
            } else if self.Hora < hora || self.Dia == dia {
                false
            } else {
                true
            }
        }
    }