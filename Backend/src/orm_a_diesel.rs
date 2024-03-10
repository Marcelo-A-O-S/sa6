
use diesel::prelude::*;
use Backend::models::*;
use mysql::*;

pub fn establish_connection() -> MysqlConnection {
    let database_url = "mysql://username:password@localhost:3306/database_name";
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

async fn add_user(user: Usuario) -> Result<Usuario, diesel::result::Error> {
    let conn = establish_connection();
    diesel::insert_into(usuarios::table)
        .values(&user)
        .execute(&conn)?;

    Ok(user)
}


async fn get_users() -> Result<Vec<Usuario>, diesel::result::Error> {
    let conn = establish_connection();
    let users = usuarios::table.load::<Usuario>(&conn)?;

    Ok(users)
}

async fn add_agendamento(agendamento: Agendamento) -> Result<Agendamento, diesel::result::Error> {
    let conn = establish_connection();
    diesel::insert_into(agendamentos::table)
        .values(&agendamento)
        .execute(&conn)?;

    Ok(agendamento)
}

async fn get_agendamentos() -> Result<Vec<Agendamento>, diesel::result::Error> {
    let conn = establish_connection();
    let agendamentos = agendamentos::table.load::<Agendamento>(&conn)?;

    Ok(agendamentos)
}

async fn add_academia(academia: Academia) -> Result<Academia, diesel::result::Error> {
    let conn = establish_connection();
    diesel::insert_into(academias::table)
        .values(&academia)
        .execute(&conn)?;

    Ok(academia)
}

async fn get_academias() -> Result<Vec<Academia>, diesel::result::Error> {
    let conn = establish_connection();
    let academias = academias::table.load::<Academia>(&conn)?;

    Ok(academias)
}

