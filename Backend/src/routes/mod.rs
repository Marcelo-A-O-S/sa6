use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
pub mod controllers;
use controllers::AcademiaController::{
    create_academia, get_academias_controller,delete_academia,update_academia, get_academia_by_id
};
use controllers::UsuariosController::{
    create_usuarios, delete_by_id, delete_usuario, get_by_id, get_usuarios, post_delete_usuario,
    update_usuario,
};
pub fn config_usuario_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_usuarios)
        .service(create_usuarios)
        .service(get_usuarios)
        .service(delete_usuario)
        .service(update_usuario)
        .service(post_delete_usuario)
        .service(delete_by_id)
        .service(get_by_id);
}
pub fn config_academia_routes(cfg: &mut web::ServiceConfig){
    cfg.service(get_academias_controller)
    .service(create_academia)
    .service(delete_academia)
    .service(update_academia)
    .service(get_academia_by_id);
}
pub async fn app_server() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .configure(config_usuario_routes)
            .configure(config_academia_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run();
    server.await
}
