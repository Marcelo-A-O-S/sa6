use crate::entities::Academia::Academia;
use super::TRepository::TRepository;
struct AcademiaRepository{

}
impl TRepository<Academia> for AcademiaRepository{
    async fn Salvar(entidade: Academia) {
        
        todo!()
    }

    async fn Listar() -> Vec<Academia> {
        todo!()
    }
}