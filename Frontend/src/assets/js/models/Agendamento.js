export class Agendamento{
    Id;
    UsuarioId;
    AcademiaId;
    Data;
    HorarioInicial;
    HorarioFechamento;
    constructor(){
        this.Id = 0;
        this.AcademiaId = 0;
        this.UsuarioId = 0;
        this.Data = new Date();
        this.HorarioInicial = "";
        this.HorarioFechamento = "";
    }
}
