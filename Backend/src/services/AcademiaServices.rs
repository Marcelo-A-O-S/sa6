use super::TServices::TServices;
use crate::entities::Academia::{Academia};
use crate::repository::AcademiaRepository::AcademiaRepository;
use crate::repository::TRepository::TRepository;
pub struct AcademiaServices{
    repository:AcademiaRepository
}
impl AcademiaServices{
    pub fn new()-> AcademiaServices{
        AcademiaServices{
            repository: AcademiaRepository::new()
        }
    }
}
impl TServices<Academia> for AcademiaServices{
    async fn Salvar(self, entidade :Academia) {
        todo!()
    }

    async fn Deletar(self, entidade: Academia) {
        todo!()
    }

    async fn Atualizar(self, entidade: Academia) {
        todo!()
    }

    async fn Listar(mut self) -> Vec<Academia> {
        let listaAcademias = self.repository.listar().await;
        listaAcademias
    }

}
