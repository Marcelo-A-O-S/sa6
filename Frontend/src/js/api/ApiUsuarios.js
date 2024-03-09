export class ApiUsuarios{
    async getUsuarios(){
        let Usuarios = await fetch("/Frontend/src/js/data/DataUsuarios.json")
        .then((response)=>{
            return response.json();
        })
        return Usuarios;
    }
}
