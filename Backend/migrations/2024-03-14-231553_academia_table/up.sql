-- Your SQL goes here
CREATE TABLE Academia (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    CapacidadeUsuarios INT NOT NULL,
    NomeComercial VARCHAR(255) NOT NULL,
    Endereco VARCHAR(255) NOT NULL,
    HorarioAbertura TIME NOT NULL,
    HorarioFechamento TIME NOT NULL
);