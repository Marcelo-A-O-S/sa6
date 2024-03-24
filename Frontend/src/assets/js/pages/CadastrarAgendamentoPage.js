
import {InputValidation,TYPEVALIDATION} from "../utils/InputValidation.js"
import {ApiAgendamento} from "../api/ApiAgendamento.js"
import {ApiUsuarios} from "../api/ApiUsuarios.js"
import {FormValidation} from "../utils/FormValidation.js"
import {ButtonValidation} from "../utils/ButtonValidation.js"
import {Agendamento} from "../models/Agendamento.js"
import {Usuario} from "../models/Usuario.js"
const params = new URLSearchParams(window.location.search);
const AcademiaId = params.get("AcademiaId");
const AgendamentoId = params.get("AgendamentoId");
const titlePage = document.getElementById("title-page")
const titleForm = document.getElementById("title-form")
console.log(AcademiaId);
console.log(AgendamentoId);
if( AcademiaId != null && AgendamentoId == null){
    titlePage.innerHTML = "Cadastrar Agendamento"
    titleForm.innerHTML = "Cadastrar Agendamento"
}
if( AcademiaId != null && AgendamentoId != null){
    titlePage.innerHTML = "Atualizar Agendamento"
    titleForm.innerHTML = "Atualizar Agendamento"
}
let usuario = new Usuario();
let usuarios = new Array()
const Meses = [
    'Janeiro', 'Fevereiro', 'Março', 'Abril', 'Maio', 'Junho', 'Julho', 'Agosto', 'Setembro', 'Outubro', 'Novembro', 'Dezembro'
]

let btnBuscarUsuario = document.getElementById("btnBuscarUsuario")
let modalUsuarios = document.getElementById("ModalUsuarios")
let btnCloseModal = document.getElementById("btnCloseModal")
let contentUsuarios = document.getElementById("content-usuario")
let btnSelecionarUsuario = document.getElementById("btnSelectUsuario")
let content;
btnBuscarUsuario.addEventListener("click", async (e)=>{
    modalUsuarios.showModal()
    const apiUsuarios = new ApiUsuarios();
    usuarios = await apiUsuarios.getUsuarios();
    console.log(usuarios)
    content = usuarios.map((item)=>{
        return(
            `
            <div class="item">
                  <input id="${item.Id}" name="opcao" class="radio" type="radio"/>
                  <label for="${item.Id}"  class="usuario-item">
                    <p>Nome: ${item.nome}</p>
                    <p>Cpf: ${item.CPF}</p>
                  </label>
                </div>
            `
        )
    })
    contentUsuarios.innerHTML = content.join("");

})
btnCloseModal.addEventListener("click",(e)=>{
    modalUsuarios.close()
})
btnSelecionarUsuario.addEventListener("click",async (e)=>{
    const apiUsuarios = new ApiUsuarios();
    let itemsUsuario = document.querySelectorAll(".radio")
    itemsUsuario.forEach(async (item)=>{
        if(item.checked == true){
            let response = await apiUsuarios.getUsuarioById(item.id);
            if(response !== undefined){
                if(response.Id !== undefined){
                    usuario.Id = response.Id;
                    usuario.nome = response.nome;
                    usuario.CPF = response.CPF;
                    inputNome.setValue(usuario.nome);
                    modalUsuarios.close();
                }
            }
        }
        
    })
    console.log(itemsUsuario)
})
let btnLastMonth = document.getElementById("pre-year");
let btnNextMonth = document.getElementById("next-year");
const inputNome = new InputValidation("nome","nomeError","nomeSucess",TYPEVALIDATION.NAME);
const inputData = new InputValidation("Data","DataError","DataSucess",TYPEVALIDATION.DATE);
const inputHorarioInicial = new InputValidation("HorarioInicial", "HorarioInicialError","HorarioInicialSucess",TYPEVALIDATION.TIME)
const inputHorarioFinal = new InputValidation("HorarioFinal", "HorarioFinalError","HorarioFinalSucess",TYPEVALIDATION.TIME)
const btnSubmit = new ButtonValidation("btnSubmitAgendamento")
const FormSubmit = new FormValidation("formAgendamentos",[inputNome,inputData,inputHorarioInicial,inputHorarioFinal],btnSubmit)
FormSubmit.onChanges();

FormSubmit.onSubmit(async ()=>{
    const apiAgendamento = new ApiAgendamento();
    let agendamento = new Agendamento();
    if(AgendamentoId != null){
        agendamento.Id =parseInt(AgendamentoId);
        agendamento.AcademiaId = parseInt(AcademiaId);
        agendamento.UsuarioId = usuario.Id;
        agendamento.HorarioFechamento = inputHorarioFinal.getValue();
        agendamento.HorarioInicial = inputHorarioInicial.getValue();
        agendamento.Data = inputData.getValue();
        console.log(agendamento)
        let response = await apiAgendamento.createAgendamento(agendamento);
        alert(response);
    }else{
        agendamento.Id = 0;
        agendamento.AcademiaId = parseInt(AcademiaId);
        agendamento.UsuarioId = usuario.Id;
        agendamento.HorarioFechamento = inputHorarioFinal.getValue();
        agendamento.HorarioInicial = inputHorarioInicial.getValue();
        agendamento.Data = inputData.getValue();
        let response = await apiAgendamento.createAgendamento(agendamento);
        alert(response);
    }
})
async function inserirDadosAtualizados(){
    let day = document.querySelectorAll(".day")
    let year = document.getElementById("year");
    let month = document.getElementById("month-picker");
    const apiAcademia = new ApiAgendamento()
        let agendamento = await apiAcademia.getAgendamentoById(AgendamentoId);
        if(agendamento != undefined){
            if(agendamento.Id != undefined){
                usuario.Id = agendamento.usuario.Id;
                usuario.nome = agendamento.usuario.nome;
                usuario.CPF = agendamento.usuario.CPF;
                inputNome.setValue(usuario.nome)
                inputHorarioFinal.setValue(agendamento.data.HoraFechamento);
                inputHorarioInicial.setValue(agendamento.data.HoraInicial);
                let dataitem = `${String(agendamento.data.Dia).padStart(2,"0")}/${String(agendamento.data.Mes).padStart(2,"0")}/${agendamento.data.Ano}`;
                inputData.setValue(dataitem);
                day.forEach((item)=>{
                    if(year.innerHTML == agendamento.data.Ano && Meses.indexOf(month.innerHTML) + 1 == agendamento.data.Mes ){
                        if(item.innerHTML == agendamento.data.Dia){
                            item.classList.add("current-day-mark")
                        }
                    }
                })
            }
        }
        FormSubmit.verifyValidation()
}
async function InsertOnClick(){
    let day = document.querySelectorAll(".day");
    day.forEach((item)=>{
        item.addEventListener("click", (e)=>{
            let dayCalender = document.querySelectorAll(".day")
            dayCalender.forEach((itemCurrent)=>{
                itemCurrent.classList.remove("current-day-mark");
            })
            if(item.innerHTML !== ""){
                item.classList.add("current-day-mark")
            }
            let dateCurrent = document.querySelector(".current-day-mark");
            let year = document.getElementById("year");
            let month = document.getElementById("month-picker");
            let numDateCurrent = dateCurrent.innerHTML;
            let numMonth = Meses.indexOf(month.innerHTML) +1;
            inputData.setValue(`${String(numDateCurrent).padStart(2,"0")}/${String(numMonth).padStart(2,"0")}/${year.innerHTML}`);
        })
    })
    
}
btnLastMonth.addEventListener("click",(e)=>{
    console.log("Mês anterior")
    InsertOnClick()

})
btnNextMonth.addEventListener("click",(e)=>{
    console.log("Proximo mês")
    InsertOnClick()
   
})
async function Initialize(){
    InsertOnClick()
    if(AgendamentoId != null){
        inserirDadosAtualizados()
    }
}

document.addEventListener("DOMContentLoaded",Initialize)