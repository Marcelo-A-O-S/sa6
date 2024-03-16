pub trait TRepository<T>{
    async fn salvar(entidade: T);
    async fn listar() -> Vec<T>;
    async fn update(entidade: T);
}