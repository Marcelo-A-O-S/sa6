use crate::schema::agendamento;
use diesel::*;

#[derive(Debug, Identifiable, Queryable, PartialEq, Clone)]
#[diesel(table_name = agendamento)]
#[diesel(primary_key(Id))]
pub struct Agendamento {
    pub Id: i32,
    pub AcademiaId: i32,
    pub UsuarioId: i32,
    pub DataHoraId: i32,
}
#[derive(Insertable)]
#[diesel(table_name = agendamento)]
pub struct NovoAgendamento {
    pub AcademiaId: i32,
    pub UsuarioId: i32,
    pub DataHoraId: i32,
}
impl Agendamento {
    /*  pub fn new(usuario: Usuario, horainicio: DataHora, horafim: DataHora) -> Self {
        let temp = Agendamento {
            usuario,
            horainicio,
            horafim,
        };
        temp.validar_inicio_e_fim();
        temp
    }
    pub fn validar_inicio_e_fim(&self) {
        if !(self.horafim.data_futura() || self.horainicio.data_futura()) {
            panic!("você só pode agendar um horario no futuro")
        }
        if !(self.horafim.ano == self.horainicio.ano
            && self.horafim.mes == self.horainicio.mes
            && self.horafim.dia == self.horainicio.dia)
        {
            panic!("o horario de inicio e fim tem que ser no mesmo dia")
        } else if self.horafim.hora < self.horainicio.hora {
            panic!("o horario de saida não pode ser antes do de entrada")
        }
    } */
}
