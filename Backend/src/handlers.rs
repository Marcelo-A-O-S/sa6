use warp::Filter;
use Backend::src::orm_a_diesel::*;
use Backend::src::models::*;

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


async fn update_agendamento(agendamento: Agendamento) -> Result<Agendamento, diesel::result::Error> {
    let conn = establish_connection();
    diesel::update(agendamentos::table)
        .filter(agendamentos::id.eq(agendamento.id))
        .set(&agendamento)
        .execute(&conn)?;

    Ok(agendamento)
}

async fn delete_agendamento(id: i32) -> Result<usize, diesel::result::Error> {
    let conn = establish_connection();
    let deleted = diesel::delete(agendamentos::table)
        .filter(agendamentos::id.eq(id))
        .execute(&conn)?;

    Ok(deleted)
}

