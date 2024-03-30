
use diesel::result::Error;
pub trait TRepository<T>{
    async fn salvar(&mut self,entidade: T)->Result<(), String>;
    async fn listar(&mut self) -> Result<Vec<T>, String>;
    async fn update(&mut self,entidade: T)->Result<(), String>;
    async fn delete(&mut self,entidade: T)->Result<(), String>;
    async fn deleteById(&mut self, _id:i32)->Result<(), String>;
    async fn findById(&mut self, _id: i32) ->Result<T, String>;
    async fn findLastId(&mut self) ->Result<T,String>;
}