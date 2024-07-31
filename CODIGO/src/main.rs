extern crate odbc;
extern crate odbc_safe;

use odbc::*;
use std::io::{self, Write};

struct Usuario {
    nome: String,
    idade: i32,
}

struct GerenciadorUsuarios {
    connection: Environment<Version3>,
}

impl GerenciadorUsuarios {
    fn new() -> Result<GerenciadorUsuarios, DiagnosticRecord> {
        let env = create_environment_v3().map_err(|e| e.unwrap()).unwrap();
        let conn = env.connect_with_connection_string(
            "DRIVER={ODBC Driver 17 for SQL Server};SERVER=DESKTOP-PK3RLSU;DATABASE=Cadastro;Trusted_Connection=yes;",
        )?;
        Ok(GerenciadorUsuarios { connection: conn })
    }

    fn adicionar_usuario(&self, nome: &str, idade: i32) -> Result<(), DiagnosticRecord> {
        let query = format!("INSERT INTO Usuarios (Nome, Idade) VALUES ('{}', {})", nome, idade);
        self.connection.execute(&query)?;
        println!("😎 USUÁRIO ADICIONADO COM SUCESSO!");
        Ok(())
    }

    fn listar_usuarios(&self) -> Result<(), DiagnosticRecord> {
        let query = "SELECT Nome, Idade FROM Usuarios";
        let stmt = self.connection.prepare(&query)?.execute()?;
        
        for result in stmt.iter() {
            match result {
                Ok(row) => {
                    let nome: &str = row.get(1).unwrap();
                    let idade: i32 = row.get(2).unwrap();
                    println!("NOME: {}, IDADE: {}", nome, idade);
                }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn atualizar_usuario(&self, nome_antigo: &str, novo_nome: &str, nova_idade: i32) -> Result<(), DiagnosticRecord> {
        let query = format!("UPDATE Usuarios SET Nome = '{}', Idade = {} WHERE Nome = '{}'", novo_nome, nova_idade, nome_antigo);
        self.connection.execute(&query)?;
        println!("😙 USUÁRIO ATUALIZADO COM SUCESSO!");
        Ok(())
    }

    fn excluir_usuario(&self, nome: &str) -> Result<(), DiagnosticRecord> {
        let query = format!("DELETE FROM Usuarios WHERE Nome = '{}'", nome);
        self.connection.execute(&query)?;
        println!("🗑 USUÁRIO EXCLUÍDO COM SUCESSO!");
        Ok(())
    }
}

fn exibir_menu() {
    println!("\nMENU:");
    println!("1. ADICIONAR USUÁRIO");
    println!("2. LISTAR USUÁRIOS");
    println!("3. ATUALIZAR USUÁRIO");
    println!("4. EXCLUIR USUÁRIO");
    println!("5. SAIR");
}

fn main() {
    let gerenciador = GerenciadorUsuarios::new().unwrap();

    loop {
        exibir_menu();
        print!("😎 ESCOLHA UMA OPÇÃO:\n>>>");
        io::stdout().flush().unwrap();

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).unwrap();
        let opcao = opcao.trim();

        match opcao {
            "1" => {
                print!("😎 DIGITE O NOME:\n>>>");
                io::stdout().flush().unwrap();
                let mut nome = String::new();
                io::stdin().read_line(&mut nome).unwrap();

                print!("😎 DIGITE A IDADE:\n>>>");
                io::stdout().flush().unwrap();
                let mut idade = String::new();
                io::stdin().read_line(&mut idade).unwrap();
                let idade: i32 = idade.trim().parse().unwrap();

                gerenciador.adicionar_usuario(nome.trim(), idade).unwrap();
            }
            "2" => {
                gerenciador.listar_usuarios().unwrap();
            }
            "3" => {
                print!("😎 DIGITE O NOME A SER ATUALIZADO:\n>>>");
                io::stdout().flush().unwrap();
                let mut nome_antigo = String::new();
                io::stdin().read_line(&mut nome_antigo).unwrap();

                print!("😎 DIGITE O NOVO NOME:\n>>>");
                io::stdout().flush().unwrap();
                let mut novo_nome = String::new();
                io::stdin().read_line(&mut novo_nome).unwrap();

                print!("😎 DIGITE A NOVA IDADE:\n>>>");
                io::stdout().flush().unwrap();
                let mut nova_idade = String::new();
                io::stdin().read_line(&mut nova_idade).unwrap();
                let nova_idade: i32 = nova_idade.trim().parse().unwrap();

                gerenciador.atualizar_usuario(nome_antigo.trim(), novo_nome.trim(), nova_idade).unwrap();
            }
            "4" => {
                print!("😎 DIGITE O NOME DO USUÁRIO A SER EXCLUÍDO:\n>>>");
                io::stdout().flush().unwrap();
                let mut nome = String::new();
                io::stdin().read_line(&mut nome).unwrap();

                gerenciador.excluir_usuario(nome.trim()).unwrap();
            }
            "5" => {
                println!("🚀 SAINDO...");
                break;
            }
            _ => {
                println!("😡 OPÇÃO INVÁLIDA. TENTE NOVAMENTE!");
            }
        }
    }
}
