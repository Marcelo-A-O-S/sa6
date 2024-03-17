use super::TServices::TServices;
use crate::entities::Academia::{Academia};
use crate::repository::AcademiaRepository::AcademiaRepository;
use crate::repository::TRepository::TRepository;
pub struct AcademiaServices{

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

    async fn Listar() -> Vec<Academia> {
        let listaAcademias = AcademiaRepository::listar().await;
        listaAcademias
    }

}
