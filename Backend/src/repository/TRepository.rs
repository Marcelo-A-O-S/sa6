pub trait TRepository<T>{
    async fn salvar(&mut self,entidade: T);
    async fn listar() -> Vec<T>;
    async fn update(&mut self,entidade: T);
}