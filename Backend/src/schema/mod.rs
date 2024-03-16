// @generated automatically by Diesel CLI.

diesel::table! {
    academia (Id) {
        Id -> Integer,
        #[max_length = 255]
        NomeComercial -> Varchar,
        #[max_length = 255]
        Endereco -> Varchar,
        HorarioAbertura -> Time,
        HorarioFechamento -> Time,
        CapacidadeUsuarios -> Integer,
    }
}
diesel::table! {
    usuario (Id) {
        Id -> Integer,
        #[max_length = 255]
        nome -> Varchar,
        #[max_length = 14]
        CPF -> Varchar,
    }
}
diesel::table! {
    academiausuarios (Id) {
        Id -> Integer,
        AcademiaId -> Nullable<Integer>,
        UsuarioId -> Nullable<Integer>,
    }
}

diesel::table! {
    agendamento (Id) {
        Id -> Integer,
        AcademiaId -> Nullable<Integer>,
        UsuarioId -> Nullable<Integer>,
        Data -> Date,
        HorarioInicial -> Time,
        HorarioFinal -> Time,
    }
}



diesel::joinable!(academiausuarios -> academia (AcademiaId));
diesel::joinable!(academiausuarios -> usuario (UsuarioId));
diesel::joinable!(agendamento -> academia (AcademiaId));
diesel::joinable!(agendamento -> usuario (UsuarioId));

diesel::allow_tables_to_appear_in_same_query!(
    academia,
    academiausuarios,
    agendamento,
    usuario,
);
