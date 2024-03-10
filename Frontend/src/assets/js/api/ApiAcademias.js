export class ApiAcademias{
    async getAcademias(){
        let Academias;
        Academias = await fetch("/Frontend/src/assets/js/data/DataAcademias.json")
        .then( (response) => {
            return response.json()
        })

        return Academias;
    }
    async getAcademiaByNomeComercial(nomeComercial){
        let Academia;
        Academia = await fetch(`/Frontend/src/js/data/DataAcademias.json?NomeComercial=${nomeComercial}`)
        .then((response)=>{
            return response.json();
        })
        return Academia;
    }
}
