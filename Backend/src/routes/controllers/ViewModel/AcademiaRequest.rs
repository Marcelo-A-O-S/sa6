use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct AcademiaRequest{
    pub Id: i32,
    pub NomeComercial: String,
    pub Endereco: String,
    pub HorarioAbertura: NaiveTime,
    pub HorarioFechamento: NaiveTime,
    pub CapacidadeUsuarios: i32,
}