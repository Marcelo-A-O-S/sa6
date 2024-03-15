pub trait TServices<T>{
    async fn Salvar(&self, entidade :T);
    async fn Deletar(&self, entidade: T);
    async fn Atualizar(&self, entidade: T);
    async fn Listar(&self) -> [T];
}