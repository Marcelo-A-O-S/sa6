create database teste;
use teste;
create table datahora(
Id int not null primary key auto_increment,
Ano int not null,
Mes int not null,
Hora time not null
);
create table usuario(
Id int not null primary key auto_increment,
nome varchar(255) not null,
CPF varchar(14) not null
);
create table academia(
Id int not null primary key auto_increment,
CapacidadeUsuarios int not null,
NomeComercial varchar(255) not null,
Endereco varchar(255) not null,
HorarioAbertura time not null,
HorarioFechamento time not null
);
use teste;
create table academiausuarios(
Id int not null primary key auto_increment,
AcademiaId int not null,
UsuarioId int not null,
foreign key(AcademiaId) references teste.academia(Id) on update cascade on delete cascade,
foreign key(UsuarioId) references teste.usuario(Id) on update cascade on delete cascade
);
use teste;
create table agendamento(
Id int primary key auto_increment not null,
AcademiaId int not null,
UsuarioId int not null,
DataHora int not null,
foreign key(AcademiaId) references teste.academia(Id) on update cascade on delete cascade,
foreign key(UsuarioId) references teste.usuario(Id) on update cascade on delete cascade,
foreign key(DataHora) references teste.datahora(Id) on update cascade on delete cascade
);