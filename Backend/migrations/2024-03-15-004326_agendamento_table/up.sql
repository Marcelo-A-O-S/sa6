-- Your SQL goes here
USE teste;
CREATE TABLE Agendamento (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    AcademiaId INT,
    UsuarioId INT,
    Data DATE NOT NULL,
    HorarioInicial TIME NOT NULL,
    HorarioFinal TIME NOT NULL,
    FOREIGN KEY (AcademiaId) REFERENCES Academia(Id),
    FOREIGN KEY (UsuarioId) REFERENCES Usuario(Id)
);