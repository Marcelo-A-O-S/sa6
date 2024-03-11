export class ApiAgendamento{
    async getAgendamentos(){
        let Agendamentos = [];
        Agendamentos = await fetch("https://marcelo-a-o-s.github.io/sa6/assets/js/data/DataAgendamentos.json")
        .then( (response) => {
            return response.json()
        })

        return Agendamentos;
    }
}
