use diesel::query_dsl::methods::FilterDsl;
use diesel::*;
use diesel::{table};
use crate::entities::Usuarios::{NovoUsuario, Usuario};
use crate::connection::estabilishConnection;
use crate::schema::{usuario, usuario::dsl::*};
use super::TRepository::TRepository;



pub struct UsuarioRepository{
    pub conn: MysqlConnection,
}
impl UsuarioRepository{
    pub fn new()-> UsuarioRepository {
        UsuarioRepository{
            conn: estabilishConnection()
        }
    }
}
impl TRepository<Usuario>  for UsuarioRepository{
    async fn salvar(&mut self, entidade: Usuario) {
        
        self.conn = estabilishConnection();
        //let conn = &mut estabilishConnection();
        
        let novo_usuario = NovoUsuario{
            CPF : entidade.CPF,
            nome: entidade.nome
        };
        diesel::insert_into(usuario::table)
        .values(&novo_usuario)
        .execute(&mut self.conn)
        .expect("Erro ao inserir dados");
        
    }
    
    async fn listar(&mut self) -> Vec<Usuario> {
        
        //let results: Vec<Usuario> = usuario.load(conn).expect("Erro em fazer consulta");
        let mut listaUsuarios: Vec<Usuario> = Vec::new();
        match usuario.load::<Usuario>(&mut self.conn){
            Ok(results)=>{
                for usuarioData in results{
                    let usuarioD = Usuario::new(i32::from(usuarioData.Id), String::from(usuarioData.nome), String::from(usuarioData.CPF));
                    listaUsuarios.push(usuarioD)
                }
                listaUsuarios
            }
            Err(err)=>{
                println!("Ocorreu o seguinte erro: {}", err);
                listaUsuarios
            }
        }
    }
    
    async fn update(&mut self,entidade: Usuario) {
        update(usuario)
        .set((CPF.eq(entidade.CPF), nome.eq(entidade.nome)))
        .filter(Id.eq(entidade.Id))
        .execute(&mut self.conn).expect("A entidade não foi atualizada com sucesso");
        
    }
    
    async fn delete(&mut self,entidade: Usuario) {
        delete(usuario)
        .filter(Id.eq(entidade.Id))
        .execute(&mut self.conn).expect("A entidade não foi deletada com sucesso");
    }
    
    async fn findById(&mut self, _id: i32) -> Usuario {
        todo!()
    }
    
    async fn deleteById(&mut self, _id:i32) {
        delete(usuario)
        .filter(Id.eq(_id))
        .execute(&mut self.conn).expect("A entidade não foi deletada com sucesso");
    }
}