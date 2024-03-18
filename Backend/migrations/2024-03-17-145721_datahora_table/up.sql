-- Your SQL goes here
use teste;
create table DataHora(
    Id INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    AgendamentoId INT NOT NULL,
    Ano int NOT NULL,
    Mes int not null,
    Hora time not null,
    FOREIGN KEY(AgendamentoId) REFERENCES teste.agendamento(Id) on  UPDATE CASCADE on DELETE CASCADE
);
use teste;
alter table teste.agendamento
drop column HorarioInicial,
drop column HorarioFinal,
drop column Data,
add column DataHoraId int not null,
add foreign key(DataHoraId) references teste.datahora(Id) on update cascade on delete cascade;
/*         pub ano: u32,
        pub mes: Mes,
        pub dia: u32,
        pub hora: NaiveTime, */