use crate::entities::{Academia::Academia, DataHora::DataHora, Usuarios::Usuario};
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct ResponseAgendamento{
    //pub academia: Academia,
    pub Id: i32,
    pub usuario: Usuario,
    pub data: DataHora
}