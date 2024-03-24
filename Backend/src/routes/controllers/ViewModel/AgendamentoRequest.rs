use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct AgendamentoRequest{
    pub Id: i32,
    pub UsuarioId: i32,
    pub AcademiaId: i32,
    pub Data: String,
    pub HorarioInicial: NaiveTime,
    pub HorarioFechamento: NaiveTime
}

