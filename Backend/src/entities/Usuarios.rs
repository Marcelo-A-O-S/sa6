<<<<<<< HEAD
use serde::{Serialize,Deserialize};
=======


#[derive( PartialEq, Debug, Clone)]
>>>>>>> e096852d21975b9a1438409ed6f96634dd56c077

use diesel::*;
use diesel::prelude::*;
use crate::schema::{usuario};
#[derive( Queryable, PartialEq, Debug, Clone)]
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
    pub fn validar_cpf(self) -> bool {
        self.CPF.chars().count() == 14
    }
}
