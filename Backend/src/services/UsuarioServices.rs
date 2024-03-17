use crate::entities::Usuarios::Usuario;
use crate::repository::{TRepository::TRepository,UsuariosRepository::UsuarioRepository};
use super::TServices::TServices;
pub struct UsuarioServices{
    repository: UsuarioRepository
}
impl UsuarioServices {
    pub fn new() -> UsuarioServices {
        return UsuarioServices { repository: UsuarioRepository::new() }
    }
}
impl TServices<Usuario> for UsuarioServices{
    
    async fn Salvar(mut self, entidade :Usuario) {
        
        if entidade.Id == 0{
            self.repository.salvar(entidade).await;
            
        }else{
            self.repository.update(entidade).await;
        }
        
    }

    async fn Deletar(self, entidade: Usuario) {
        todo!()
    }

    async fn Atualizar(self, entidade: Usuario) {
        todo!()
    }

    async fn Listar() -> Vec<Usuario> {
        let listaUsuarios = UsuarioRepository::listar().await;
        listaUsuarios
    }
}