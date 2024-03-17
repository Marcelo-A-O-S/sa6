use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
pub fn estabilishConnection() -> MysqlConnection{
    dotenv().ok();

    let database_url ="mysql://root:654321@127.0.0.1:3306/teste?ssl_mode=DISABLED"; //env::var("DATABASE_URL").expect("Variavel de ambiente não configurada");
    MysqlConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
