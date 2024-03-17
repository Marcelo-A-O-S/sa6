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
    
    async fn listar() -> Vec<Usuario> {
        let conn = &mut estabilishConnection();
        //let results: Vec<Usuario> = usuario.load(conn).expect("Erro em fazer consulta");
        let mut listaUsuarios: Vec<Usuario> = Vec::new();
        match usuario.load::<Usuario>(conn){
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
        
       /*  update(usuario::table.filter(entidade.Id))
        .set() */
        todo!()
    }
}