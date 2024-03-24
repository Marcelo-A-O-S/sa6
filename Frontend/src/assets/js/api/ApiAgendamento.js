export class ApiAgendamento{
    async getAgendamentos(){
        let Agendamentos = [];
        Agendamentos = await fetch("https://marcelo-a-o-s.github.io/sa6/assets/js/data/DataAgendamentos.json")
        .then( (response) => {
            return response.json()
        })

        return Agendamentos;
    }
    async createAgendamento(agendamento){
        return await fetch("http://localhost:8080/CreateAgendamento",{
            body: JSON.stringify(agendamento),
            method: 'POST',
            mode: "cors",
            headers: {
                'Content-Type': 'application/json'
            }
        }).then((response)=> response.json())
        .then((data)=>{
            console.log(data);
            return data;
        })
    }
    async getAgendamentoById(agendamentoId){
        return await fetch(`http://localhost:8080/GetAgendamentoById/${agendamentoId}`)
        .then((response)=> response.json())
        .then((data)=>{
            console.log(data);
            return data;
        })
    }
    async DeleteAgendamentoById(Id){
        return await fetch(`http://localhost:8080/DeleteAgendamentoById/${Id}`, {
            method: "DELETE"
        })
        .then((response) =>  response.json())
        .then((data) => { return data});
    }
}
