
use diesel::result::Error;
use diesel::*;
use diesel::{table};
use crate::entities::Usuarios::{NovoUsuario, Usuario};
use crate::connection::estabilishConnection;
use crate::schema::{usuario, usuario::dsl::*};
use super::TRepository::TRepository;


use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::*;


pub struct UsuarioRepository{
    pub conn: MysqlConnection,
}
impl UsuarioRepository{
    pub fn new()-> UsuarioRepository {
        UsuarioRepository{
            conn: estabilishConnection()
        }
    }
    pub async fn findByCPF(&mut self, _cpf: &String) -> Result<Usuario, String> {
        let result:Result<Usuario,Error> = usuario::table
        .filter(usuario::CPF.eq(_cpf))
        .first::<Usuario>(&mut self.conn);
        match result {
            Ok(usuario_result )=>{
                return Ok(usuario_result);
            }
            Err(e)=>{
                return Err(String::from("Erro ao buscar dado"));
            }
        }
    }
}
impl TRepository<Usuario>  for UsuarioRepository{
    async fn salvar(&mut self, entidade: Usuario) ->Result<(), String>{
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
            Err(String::from("Erro ao salvar o dado"))
        }
    }
    
    async fn listar(&mut self) -> Result<Vec<Usuario>, String>  {
        
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
                return Err(String::from("Houve um problema ao puxar a lista de entidade"))
            }
        }
    }
    
    async fn update(&mut self,entidade: Usuario) -> Result<(), String>{
        let result = update(usuario)
        .set((CPF.eq(entidade.CPF), nome.eq(entidade.nome)))
        .filter(Id.eq(entidade.Id))
        .execute(&mut self.conn);
        if result.is_ok(){
            Ok(())
        }else{
            Err(String::from("Erro ao atualizar dados"))
        }
    }
    
    async fn delete(&mut self,entidade: Usuario) ->Result<(), String>{
        let result = delete(usuario)
        .filter(Id.eq(entidade.Id))
        .execute(&mut self.conn);
        if result.is_ok(){
            Ok(())
        }else{
            Err(String::from("Error ao deletar entidade"))
        }
    }
    
    async fn findById(&mut self, _id: i32) -> Result<Usuario, String> {
        let result:Result<Usuario,Error> = usuario::table
        .filter(usuario::Id.eq(_id))
        .first::<Usuario>(&mut self.conn);
        match result {
            Ok(usuario_result )=>{
                return Ok(usuario_result);
            }
            Err(e)=>{
                return Err(String::from("Erro ao buscar dado"))
            }
        }
    }
    
    async fn deleteById(&mut self, _id:i32) -> Result<(), String>{
        let result = delete(usuario)
        .filter(Id.eq(_id))
        .execute(&mut self.conn);
        if result.is_ok(){
            return Ok(());
        }else{
            return Err(String::from("Erro ao deletar entidade"))
        }
    }
    
    async fn findLastId(&mut self) ->Result<Usuario,String> {
        let result = usuario
        .select(usuario::all_columns)
        .order(Id.desc())
        .limit(1)
        .first::<Usuario>(&mut self.conn);
        match result {
            Ok(user)=>{
                return Ok(user)
            }
            Err(err)=>{
                return Err(String::from("Erro nesse baguio"))
            }
        }
    }

    
}