export class ApiUsuarios{
    async getUsuarios(){
        let Usuarios = await fetch("https://marcelo-a-o-s.github.io/sa6/assets/js/data/DataUsuarios.json")
        .then((response)=>{
            return response.json();
        })
        return Usuarios;
    }
}
