CREATE DATABASE Cadastro;
GO

USE Cadastro;
GO

CREATE TABLE Usuarios (
    Id INT IDENTITY(1,1) PRIMARY KEY,
    Nome NVARCHAR(255) NOT NULL,
    Idade INT
);
GO
