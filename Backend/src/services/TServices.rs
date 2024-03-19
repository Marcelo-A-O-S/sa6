use std::error::Error;
use crate::utils::Errors::{ErrorProject};
pub trait TServices<T>{
    async fn Salvar(&mut self, _entidade :T)-> Result<(),ErrorProject>;
    async fn Deletar(&mut self, _entidade: T) -> Result<(),ErrorProject>;
    async fn Atualizar(&mut self, _entidade: T) ->Result<(),ErrorProject>;
    async fn Listar(&mut self) -> Result<Vec<T>,ErrorProject>;
    async fn BuscarPorId(&mut self, _id: i32)->Result<T,ErrorProject>;
    async fn DeletarPorId(&mut self,_id: i32) ->Result<(),ErrorProject>;
}