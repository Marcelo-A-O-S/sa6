use crate::entities::Usuarios::Usuario;
use crate::repository::{TRepository::TRepository,UsuariosRepository::UsuarioRepository};
use super::TServices::TServices;
pub struct UsuarioServices{
}
impl TServices<Usuario> for UsuarioServices{
    
    async fn Salvar(&self, entidade :Usuario) {
        if entidade.Id == 0{
            UsuarioRepository::salvar(entidade).await
        }else{
            UsuarioRepository::update(entidade).await
        }
        
    }

    async fn Deletar(&self, entidade: Usuario) {
        todo!()
    }

    async fn Atualizar(&self, entidade: Usuario) {
        todo!()
    }

    async fn Listar(&self) -> Vec<Usuario> {
        let listaUsuarios = UsuarioRepository::listar().await;
        listaUsuarios
    }
}