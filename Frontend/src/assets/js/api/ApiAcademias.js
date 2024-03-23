export class ApiAcademias{
    async getAcademias(){
        let Academias;
        Academias = await fetch("http://localhost:8080/Academias")
        .then( (response) => {
            return response.json()
        })

        return Academias;
    }
    async getAcademiaByNomeComercial(nomeComercial){
        let Academia = [];
        Academia = await fetch(`/Frontend/src/js/data/DataAcademias.json?NomeComercial=${nomeComercial}`)
        .then((response)=>{
            return response.json();
        })
        return Academia;
    }
    async getAcademiaById(Id){
        return await fetch(`http://localhost:8080/AcademiaGetById/${Id}`)
        .then((response)=> response.json())
        .then((data) => {return data})
    }
}
