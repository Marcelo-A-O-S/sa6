export class Agendamento{
    UsuarioId;
    AcademiaId;
    Data;
    HorarioInicial;
    HorarioFinal;
    constructor(){
        this.AcademiaId = 0;
        this.UsuarioId = 0;
        this.Data = new Date();
        this.HorarioInicial = "";
        this.HorarioFinal = "";
    }
}
