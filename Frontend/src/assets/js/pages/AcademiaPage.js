
import {ApiAgendamento} from "../api/ApiAgendamento.js"
const params = new URLSearchParams(window.location.search);
const NomeComercial = params.get("NomeComercial");
console.log(NomeComercial);
document.getElementById("title-page").innerHTML = NomeComercial;
document.getElementById("title-academia").innerText = NomeComercial;
const contentPageAgendamentos = document.getElementById("content-agendamentos")
async function LoadAgendamentos(){
    let Agendamentos;
    let contentAgendamentos;
    const apiAgendamentos = new ApiAgendamento();
    Agendamentos = await apiAgendamentos.getAgendamentos();
    contentAgendamentos = Agendamentos.map((item)=>{
        return(
            `
            <tr class="">
                <td class="text-light">${item.Nome}</td>
                <td class="text-light">${item.Data}</td>
                <td class="text-light">${item.HorarioInicial}</td>
                <td class="text-light">${item.HorarioFinal}</td>
                <td class="d-flex align-items-center gap-4" colspan="3">
                    <button onclick="window.location.href='pag10.html'" class="btn-custom">Atualizar</button>
                    <button onclick="window.location.href='pag10.html'" class="btn btn-danger">Remover</button>
                </td>
            </tr>
            `
            )
    })
    contentPageAgendamentos.innerHTML = contentAgendamentos.join("")
}
document.addEventListener("DOMContentLoaded", LoadAgendamentos);
