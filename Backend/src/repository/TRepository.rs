use crate::schema::academia::Id;

pub trait TRepository<T>{
    async fn salvar(&mut self,entidade: T);
    async fn listar(&mut self) -> Vec<T>;
    async fn update(&mut self,entidade: T);
    async fn delete(&mut self,entidade: T);
    async fn deleteById(&mut self, _id:i32);
    async fn findById(&mut self, _id: i32) -> T;
}