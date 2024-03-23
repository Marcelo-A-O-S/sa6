
import {ApiAgendamento} from "../api/ApiAgendamento.js"
import {ApiAcademias} from "../api/ApiAcademias.js"
const params = new URLSearchParams(window.location.search);
const NomeComercial = params.get("NomeComercial");
const AcademiaId = params.get("Id");

const contentPageAgendamentos = document.getElementById("content-agendamentos")
async function LoadAgendamentos(){
    let Agendamentos;
    let contentAgendamentos;
    let academia;
    const apiAcademia = new ApiAcademias();
    const apiAgendamentos = new ApiAgendamento();
    academia = await apiAcademia.getAcademiaById(parseInt(AcademiaId))
    if(academia != undefined){
        if(academia.Id != undefined){
            document.getElementById("title-page").innerHTML = academia.NomeComercial;
            document.getElementById("title-academia").innerText = academia.NomeComercial;
        }
    }
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
                    <button onclick="window.location.href='CriarAgendamento.html?'" class="btn-custom">Atualizar</button>
                    <button onclick="window.location.href='pag10.html'" class="btn btn-danger">Remover</button>
                </td>
            </tr>
            `
            )
    })
    contentPageAgendamentos.innerHTML = contentAgendamentos.join("")
}
document.addEventListener("DOMContentLoaded", LoadAgendamentos);
