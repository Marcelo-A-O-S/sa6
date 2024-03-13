#[derive(Debug)]
struct TesteGenerico<T>{
    atributo1: T,
    atributo2: T,

}

impl<T: std::fmt::Debug> TesteGenerico<T>{
    fn teste(self){
        println!("{:?}", self.atributo1);
    }
}
fn main() {
    let testeGenerico = TesteGenerico{atributo1: "Teste", atributo2: "Teste2"};
    testeGenerico.teste();
}


