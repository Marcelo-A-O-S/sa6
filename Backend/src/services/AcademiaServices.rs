use super::TServices::TServices;
use crate::entities::Academia::{Academia};
pub struct AcademiaServices{

}
impl TServices<Academia> for AcademiaServices{
    async fn Salvar(&self, entidade :Academia) {
        todo!()
    }

    async fn Deletar(&self, entidade: Academia) {
        todo!()
    }

    async fn Atualizar(&self, entidade: Academia) {
        todo!()
    }

    async fn Listar(&self) -> Vec<Academia> {
        todo!()
    }

}
