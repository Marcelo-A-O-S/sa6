use chrono::prelude::*;

#[derive(Clone, PartialEq, PartialOrd)]
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

#[derive(Clone)]
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
        obj.validar_espaco_tempo_continuo();
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
            _ => panic!("alguma coisa deu errado..."),
        };

        if self.dia > ultimo_dia_possivel {
            panic!("esse dia não existe...")
        }

        match self.hora {
            0..=24 => {}
            _ => panic!("que horas?"),
        }
    }
    pub fn validar_espaco_tempo_continuo(&self) {
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

        if self.ano < ano || mesmarcado < mes || self.dia < dia || self.hora <= hora {
            panic!("exercicio do de volta ao futuro?")
        }
    }
}

pub struct Usuario {
    pub id: i32,
    pub nome: String,
    pub senha: String,
}

impl Usuario {
    pub fn new(id: i32, nome: String, senha: String) -> Self {
        Self { id, nome, senha }
    }
}

/*

pub struct Horario {
    pub entrada: DateTime,
    pub saida: DateTime,
    pub usuario: String,
    pub status: String,
}

impl Horario {
    pub fn new(entrada: DateTime, saida: DateTime, usuario: String, status: String) -> Self {
        Horario {
            entrada,
            saida,
            usuario,
            status,
        }
    }

    pub fn validar_horario(self) {

    }
}

*/
