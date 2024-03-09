-- Tabela Usuario
CREATE TABLE Usuario (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    nome VARCHAR(255) NOT NULL,
    CPF VARCHAR(14) NOT NULL UNIQUE
);

-- Tabela Academia
CREATE TABLE Academia (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    CapacidadeUsuarios INT NOT NULL,
    NomeComercial VARCHAR(255) NOT NULL,
    Endereco VARCHAR(255) NOT NULL,
    HorarioAbertura TIME NOT NULL,
    HorarioFechamento TIME NOT NULL
);

-- Tabela AcademiaUsuarios
CREATE TABLE AcademiaUsuarios (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    AcademiaId INT,
    UsuarioId INT,
    FOREIGN KEY (AcademiaId) REFERENCES Academia(Id),
    FOREIGN KEY (UsuarioId) REFERENCES Usuario(Id)
);

-- Tabela Agendamento
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
