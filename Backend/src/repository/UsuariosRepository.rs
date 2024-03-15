use crate::entities::Usuarios::{NovoUsuario, Usuario};
use crate::connection::estabilishConnection;
use crate::schema::{usuario, usuario::dsl::*};
use super::TRepository::TRepository;

use diesel::*;
use diesel::prelude::*;

struct UsuarioRepository{

}

impl TRepository<Usuario>  for UsuarioRepository{
    async fn Salvar(entidade: Usuario) {
    
        let conn = &mut estabilishConnection();
        
        let novo_usuario = NovoUsuario{
            CPF : entidade.CPF,
            nome: entidade.nome
        };
        diesel::insert_into(usuario::table)
        .values(&novo_usuario)
        .execute(conn)
        .expect("Erro ao inserir dados");
    }
    
    async fn Listar() -> Vec<Usuario> {
        let conn = &mut estabilishConnection();
        let results: Vec<Usuario> = usuario.load(conn).expect("Erro em fazer consulta");
        let mut listaUsuarios: Vec<Usuario> = Vec::new();
        for usuarioData in results{
            let usuarioD = Usuario::new(i32::from(usuarioData.Id), String::from(usuarioData.nome), String::from(usuarioData.CPF));
            listaUsuarios.push(usuarioD)
        }
        listaUsuarios
    }
}