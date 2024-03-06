#[cfg(test)]
mod tests {
    use crate::backend::*;

    #[test]
    #[should_panic]
    fn data_impossivel() {
        let data_impossivel: DataHora = DataHora::new(2024, Mes::Fevereiro, 30, 12);
        data_impossivel.validar_data();
    }

    #[test]
    #[should_panic]
    fn hora_impossivel() {
        let _hora_impossivel: DataHora = DataHora::new(2024, Mes::Setembro, 25, 69);
    }

    #[test]
    fn hora_normal() {
        let hora_normal: DataHora = DataHora::new(2024, Mes::Setembro, 30, 15);
        hora_normal.validar_data()
    }

    #[test]
    fn reconhece_ano_bissexto() {
        let ano_bissexto: DataHora = DataHora::new(2024, Mes::Fevereiro, 29, 20);
        ano_bissexto.validar_data()
    }

    #[test]
    #[should_panic]
    fn reconhece_ano_nao_bissexto() {
        let ano_nao_bissexto: DataHora = DataHora::new(2023, Mes::Fevereiro, 29, 20);
        ano_nao_bissexto.validar_data()
    }

    #[test]
    fn reconhece_passado() {
        let data_passada: DataHora = DataHora::new(2000, Mes::Dezembro, 1, 20);
        assert_eq!(data_passada.data_futura(), false)
    }

    #[test]
    fn reconhece_futuro() {
        let data_futura: DataHora = DataHora::new(369420, Mes::Dezembro, 30, 23);
        assert_eq!(data_futura.data_futura(), true)
    }

    #[test]
    #[should_panic]
    fn agendar_no_passado_da_erro() {
        let godoberto: Usuario =
            Usuario::new(String::from("godoberto"), String::from("123.456.789-10"));
        let data_inicio: DataHora = DataHora::new(2019, Mes::Fevereiro, 20, 5);
        let data_fim: DataHora = DataHora::new(2019, Mes::Fevereiro, 20, 7);

        let _agendamento_no_passado: Agendamento =
            Agendamento::new(godoberto, data_inicio, data_fim);
    }

    #[test]
    fn agendar_normal() {
        let godoberto: Usuario =
            Usuario::new(String::from("godoberto"), String::from("123.456.789-10"));
        let data_inicio: DataHora = DataHora::new(2077, Mes::Marco, 20, 5);
        let data_fim: DataHora = DataHora::new(2077, Mes::Marco, 20, 7);

        let _agendamento_normal: Agendamento = Agendamento::new(godoberto, data_inicio, data_fim);
    }

    #[test]
    #[should_panic]
    fn agendar_mais_que_um_dia() {
        let godoberto: Usuario =
            Usuario::new(String::from("godoberto"), String::from("123.456.789-10"));
        let data_inicio: DataHora = DataHora::new(69420, Mes::Marco, 20, 5);
        let data_fim: DataHora = DataHora::new(69420, Mes::Marco, 21, 7);
        let _agendamento_falho: Agendamento = Agendamento::new(godoberto, data_inicio, data_fim);
    }

    #[test]
    #[should_panic]
    fn agendar_inicio_depois_do_fim() {
        let godoberto: Usuario =
            Usuario::new(String::from("godoberto"), String::from("123.456.789-10"));
        let data_inicio: DataHora = DataHora::new(69420, Mes::Marco, 20, 7);
        let data_fim: DataHora = DataHora::new(69420, Mes::Marco, 20, 5);
        let _agendamento_normal: Agendamento = Agendamento::new(godoberto, data_inicio, data_fim);
    }

    #[test]
    fn cpf_valido() {
        let godoberto: Usuario =
            Usuario::new(String::from("godoberto"), String::from("123.456.789-10"));
        assert_eq!(godoberto.validar_cpf(), true)
    }

    #[test]
    fn cpf_invalido() {
        let godoberto: Usuario =
            Usuario::new(String::from("godoberto"), String::from("12345678910"));
        assert_eq!(godoberto.validar_cpf(), false)
    }
}
