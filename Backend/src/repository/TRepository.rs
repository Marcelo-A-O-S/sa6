pub trait TRepository<T>{
    async fn Salvar(entidade: T);
    async fn Listar() -> Vec<T>;
}