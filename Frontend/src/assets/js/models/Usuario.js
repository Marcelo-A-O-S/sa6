export class Usuario{
    Id;
    nome;
    CPF;
    constructor(_name = "", _cpf = "", _Id =0){
        this.nome = _name;
        this.CPF = _cpf;
        this.Id = _Id
    }
}
