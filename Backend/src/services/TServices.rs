pub trait TServices<T>{
    async fn Salvar(self, _entidade :T);
    async fn Deletar(self, _entidade: T);
    async fn Atualizar(self, _entidade: T);
    async fn Listar(self) -> Vec<T>;
}