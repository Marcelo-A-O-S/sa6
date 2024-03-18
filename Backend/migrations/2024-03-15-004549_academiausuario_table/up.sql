-- Your SQL goes here
Use teste;
CREATE TABLE AcademiaUsuarios (
    Id INT AUTO_INCREMENT PRIMARY KEY,
    AcademiaId INT,
    UsuarioId INT,
    FOREIGN KEY (AcademiaId) REFERENCES Academia(Id),
    FOREIGN KEY (UsuarioId) REFERENCES Usuario(Id)
);