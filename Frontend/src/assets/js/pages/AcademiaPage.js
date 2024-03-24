
import {ApiAgendamento} from "../api/ApiAgendamento.js"
import {ApiAcademias} from "../api/ApiAcademias.js"
const params = new URLSearchParams(window.location.search);
const AcademiaId = params.get("Id");
document.addEventListener("click",(e)=>{
    if(e.target.matches("#btnAgendamento")){
        window.location.href=`CriarAgendamento.html?AcademiaId=${AcademiaId}`
    }
})
document.addEventListener('click', async function(event) {
    if (event.target.matches('#btnDelete')) {
      console.log(event.target.dataset.id)
      const apiAgendamento = new ApiAgendamento();
      let response = await apiAgendamento.DeleteAgendamentoById(event.target.dataset.id)
      console.log(response);
      window.location.href=""
    }
  });
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

    Agendamentos = await apiAcademia.getAgendamentosByAcademiaId(academia.Id)
    contentAgendamentos = Agendamentos.map((item)=>{
        let dataitem = `${String(item.data.Dia).padStart(2,"0")}/${String(item.data.Mes).padStart(2,"0")}/${item.data.Ano}`
        return(
            `
            <tr class="">
                <td class="text-light">${item.Id}</td>
                <td class="text-light">${item.usuario.nome}</td>
                <td class="text-light">${dataitem}</td>
                <td class="text-light">${item.data.HoraInicial}</td>
                <td class="text-light">${item.data.HoraFechamento}</td>
                <td class="d-flex align-items-center gap-4" colspan="3">
                    <button onclick="window.location.href='CriarAgendamento.html?AcademiaId=${AcademiaId}&AgendamentoId=${item.Id}'" class="btn-custom">Atualizar</button>
                    <button data-id=${item.Id} Id="btnDelete"  class="btn btn-danger">Remover</button>
                </td>
            </tr>
            `
            )
    })
    contentPageAgendamentos.innerHTML = contentAgendamentos.join("")
}
document.addEventListener("DOMContentLoaded", LoadAgendamentos);
