
use diesel::result::Error;
pub trait TRepository<T>{
    async fn salvar(&mut self,entidade: T)->Result<(), Error>;
    async fn listar(&mut self) -> Result<Vec<T>, Error>;
    async fn update(&mut self,entidade: T)->Result<(), Error>;
    async fn delete(&mut self,entidade: T)->Result<(), Error>;
    async fn deleteById(&mut self, _id:i32)->Result<(), Error>;
    async fn findById(&mut self, _id: i32) ->Result<T, Error>;
}