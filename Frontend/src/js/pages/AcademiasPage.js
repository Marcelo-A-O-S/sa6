import { ApiAcademias } from "../api/ApiAcademias.js"

async function loadAcademias(){

    const apiAcademias = new ApiAcademias()
    let Academias=[];
    let content;
    const contentAcademias = document.getElementById("content-academias")
    Academias = await apiAcademias.getAcademias();
    content = Academias.map((item)=>{
        return(
            `
            <div class="academia-item">
                <h1 >${item.NomeComercial}</h1>
                <p>Endereço : ${item.Endereco}</p>
                <ul id="academy1List" class="list-group">
                </ul>
                <div>
                    <button onclick="window.location.href='Academia.html?NomeComercial=${item.NomeComercial}'" class="btn btn-primary">Entrar</button>
                </div>
            </div>
            <hr>
            `
        )
    })
    contentAcademias.innerHTML += content.join("")
}
document.addEventListener('DOMContentLoaded', loadAcademias);

