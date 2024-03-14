use super::Usuarios::Usuario;
use super::DataHora::DataHora;
#[derive(PartialEq, Debug, Clone)]
    pub struct Agendamento {
        pub usuario: Usuario,
        pub horainicio: DataHora,
        pub horafim: DataHora,
    }

    impl Agendamento {
        pub fn new(usuario: Usuario, horainicio: DataHora, horafim: DataHora) -> Self {
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
        }
    }