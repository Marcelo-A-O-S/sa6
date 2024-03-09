import {ApiAcademias} from "../api/ApiAcademias.js"
const params = new URLSearchParams(window.location.search);
const NomeComercial = params.get("NomeComercial");
console.log(NomeComercial);
document.getElementById("title-page").innerHTML = NomeComercial;
document.getElementById("title").innerText = NomeComercial;

async function Load(){
    let Academia;
    const apiAcademia = new ApiAcademias();
    Academia = await apiAcademia.getAcademiaByNomeComercial(NomeComercial);
    console.log(Academia);
}
document.addEventListener("DOMContentLoaded", Load);
