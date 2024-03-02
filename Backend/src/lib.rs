pub struct Usuario {
    pub id: i32,
    pub nome: String,
    pub senha: String,
}

impl Usuario {
    pub fn new(id: i3, nome: String, senha: String) -> Self {
        Self{
            id,
            nome,
            senha, }
    }

    pub fn login(nome: String, senha: String) -> Self {
        todo!() // validar no banco de dados
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