export class Academia{
    CapacidadeUsuarios;
    NomeComercial;
    Endereco;
    HorarioAbertura;
    HorarioFechamento;
    constructor(){
        this.CapacidadeUsuarios = 0;
        this.NomeComercial = "";
        this.Endereco = "";
        this.HorarioAbertura = new Date;
        this.HorarioFechamento = new Date;
    }
}
