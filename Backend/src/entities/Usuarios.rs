
use serde::{Serialize,Deserialize};
use diesel::*;
use crate::schema::{usuario};
#[derive(Queryable, Selectable,Identifiable, PartialEq, Serialize, Debug, Clone)]
#[diesel(table_name = usuario)]
#[diesel(primary_key(Id))]
pub struct Usuario {
    pub Id: i32,
    pub nome: String,
    pub CPF: String,
}
#[derive(Insertable)]
#[diesel(table_name = usuario)]
pub struct NovoUsuario {
    pub nome: String,
    pub CPF: String
}
impl Usuario {
    pub fn new(id: i32, nome: String, cpf: String) -> Usuario {
        Usuario { Id: id ,nome: nome, CPF: cpf}
    }
    pub fn validar_cpf(&mut self) -> bool {
        self.CPF.chars().count() == 14
    }
}
