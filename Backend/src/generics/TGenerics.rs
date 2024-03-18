//use diesel::Table;
//use mysql::Conn;

pub trait TGenerics<T>{
    //async fn Salvar(_conn: Conn,_table: dyn Table, _entidade: T);
    async fn listar(_entidade: T);
}