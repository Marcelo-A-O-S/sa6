export class ApiAcademias{
    async getAcademias(){
        let Academias;
        Academias = await fetch("https://marcelo-a-o-s.github.io/sa6/assets/js/data/DataAcademias.json")
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
}
