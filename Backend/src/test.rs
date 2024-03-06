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
        let hora_impossivel: DataHora = DataHora::new(2024, Mes::Setembro, 25, 69);
        hora_impossivel.validar_data();
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
}
