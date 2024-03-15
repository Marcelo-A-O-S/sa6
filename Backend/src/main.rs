


mod repository;
mod entities;
mod routes;

use entities::Usuarios::Usuario;

fn main(){
    let user = Usuario{
        cpf: String::from("Teste"),
        nome: String::from("Henrique")
    };

    println!("Nome do meliante: {:?}", user.nome);
}


