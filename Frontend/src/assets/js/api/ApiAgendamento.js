export class ApiAgendamento{
    async getAgendamentos(){
        let Agendamentos;
        Agendamentos = await fetch("/Frontend/src/assets/js/data/DataAgendamentos.json")
        .then( (response) => {
            return response.json()
        })

        return Agendamentos;
    }
}
