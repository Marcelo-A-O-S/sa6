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

    async fn Deletar(mut self, entidade: Usuario) {
        self.repository.delete(entidade).await;
    }

    async fn Atualizar(mut self, entidade: Usuario) {
        self.repository.update(entidade).await
    }

    async fn Listar(mut self) -> Vec<Usuario> {
        let listaUsuarios = self.repository.listar().await;
        listaUsuarios
    }
}