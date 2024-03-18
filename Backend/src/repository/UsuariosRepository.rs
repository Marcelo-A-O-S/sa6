
use diesel::result::Error;
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
    async fn salvar(&mut self, entidade: Usuario) ->Result<(), Error>{
        let novo_usuario = NovoUsuario{
            CPF : entidade.CPF,
            nome: entidade.nome
        };
        let result = diesel::insert_into(usuario::table)
        .values(&novo_usuario)
        .execute(&mut self.conn);
        if result.is_ok(){
            Ok(())
        }else{
            Err(NotFound)
        }
    }
    
    async fn listar(&mut self) -> Result<Vec<Usuario>, Error>  {
        
        //let results: Vec<Usuario> = usuario.load(conn).expect("Erro em fazer consulta");
        let mut listaUsuarios: Vec<Usuario> = Vec::new();
        match usuario.load::<Usuario>(&mut self.conn){
            Ok(results)=>{
                for usuarioData in results{
                    let usuarioD = Usuario::new(i32::from(usuarioData.Id), String::from(usuarioData.nome), String::from(usuarioData.CPF));
                    listaUsuarios.push(usuarioD)
                }
                Ok(listaUsuarios)
            }
            Err(err)=>{
                println!("Ocorreu o seguinte erro: {}", err);
                return Err(NotFound)
            }
        }
    }
    
    async fn update(&mut self,entidade: Usuario) -> Result<(), Error>{
        let result = update(usuario)
        .set((CPF.eq(entidade.CPF), nome.eq(entidade.nome)))
        .filter(Id.eq(entidade.Id))
        .execute(&mut self.conn);
        if result.is_ok(){
            Ok(())
        }else{
            Err(NotFound)
        }
    }
    
    async fn delete(&mut self,entidade: Usuario) ->Result<(), Error>{
        let result = delete(usuario)
        .filter(Id.eq(entidade.Id))
        .execute(&mut self.conn);
        if result.is_ok(){
            Ok(())
        }else{
            Err(()).expect("Error ao deletar entidade")
        }
    }
    
    async fn findById(&mut self, _id: i32) -> Result<Usuario, Error> {
        let usuario_table = usuario;

        let usuario_body = usuario.filter(Id.eq(_id))
        .first::<Usuario>(&mut self.conn);
        return Err(()).expect("Error")
    }
    
    async fn deleteById(&mut self, _id:i32) -> Result<(), Error>{
        let result = delete(usuario)
        .filter(Id.eq(_id))
        .execute(&mut self.conn);
        if result.is_ok(){
            return Ok(());
        }else{
            return Err(()).expect("Erro ao deletar entidade");
        }
    }
}