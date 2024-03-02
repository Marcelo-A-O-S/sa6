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

pub struct DataHora {
    ano: u32,
    mes: Mes,
    dia: u32,
    hora: u32,
    minuto: u32,
}

impl DataHora {
    pub fn new(ano: u32, mes: Mes, dia: u32, hora: u32, minuto: u32) -> Self {
        DataHora {
            ano,
            mes,
            dia,
            hora,
            minuto,
        }
    }

    pub fn validar_data(self) {
        let mut ultimo_dia_possivel: u32 = 0;

        ultimo_dia_possivel = match self.mes {
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

/*
    pub fn login(nome: String, senha: String) -> Self {
        todo!() // validar no banco de dados
    }
    */
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
