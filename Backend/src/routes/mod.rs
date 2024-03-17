use actix_web::{App, HttpServer, web};
use actix_cors::Cors;
pub mod controllers;
use controllers::AcademiaController::
{
    get_academias_controller,
    create_academia
};
use controllers::UsuariosController::
{
    create_usuarios,
    get_usuarios,
    delete_usuario,
    post_delete_usuario,
    update_usuario
};
/* pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_academias_controller)
    .service(create_usuarios);
} */


pub async fn app_server() -> std::io::Result<()> {
    let server = HttpServer::new(||{
        let cors = Cors::permissive();
        App::new()
        .wrap(cors)
        //Rotas Academia
        .service(get_academias_controller)
        .service(create_academia)
        //Rotas Usuario
        .service(create_usuarios)
        .service(get_usuarios)
        .service(delete_usuario)
        .service(update_usuario)
        .service(post_delete_usuario)
    })
    .bind(("127.0.0.1",8080))?
    .run();
    server.await
}


