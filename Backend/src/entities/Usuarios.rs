use serde::{Serialize,Deserialize};
use diesel;
#[derive( PartialEq, Debug, Clone)]

pub struct Usuario {
    pub nome: String,
    pub cpf: String,
}

impl Usuario {
    pub fn new(&self, nome: String, cpf: String) -> Usuario {
        Usuario { nome: nome, cpf: cpf}
    }
    pub fn validar_cpf(self) -> bool {
        self.cpf.chars().count() == 14
    }
}
