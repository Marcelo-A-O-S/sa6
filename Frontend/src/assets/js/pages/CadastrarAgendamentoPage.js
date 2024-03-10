import {InputValidation,TYPEVALIDATION} from "../utils/InputValidation.js"
import {ApiAgendamento} from "../api/ApiAgendamento.js"
import {FormValidation} from "../utils/FormValidation.js"
import {ButtonValidation} from "../utils/ButtonValidation.js"
let day = document.querySelectorAll(".day")
let year = document.getElementById("year")
const inputNome = new InputValidation("nome","nomeError","nomeSucess",TYPEVALIDATION.NAME);
const inputDia = new InputValidation("Dia","DiaError","DiaSucess",TYPEVALIDATION.DATE);
const inputHorario = new InputValidation("Horario", "HorarioError","HorarioSucess",TYPEVALIDATION.TIME)
const btnSubmit = new ButtonValidation("btnSubmitAgendamento")
const FormSubmit = new FormValidation("formAgendamentos",[inputNome,inputDia,inputHorario],btnSubmit)
FormSubmit.onChanges();
let testeData = new Date(2024,1,2);
console.log(testeData)
async function Initialize(){
    const apiAgendamentos = new ApiAgendamento();
    let agendamentos = await apiAgendamentos.getAgendamentos();
    day.forEach((item)=>{
        item.addEventListener("click",(e)=>{
            let dayCalender = document.querySelectorAll(".day")
            dayCalender.forEach((itemCurrent)=>{
                itemCurrent.classList.remove("current-day-mark");
            })
            if(item.innerHTML !== ""){
                item.classList.add("current-day-mark")
            }


        })

    })
}

document.addEventListener("DOMContentLoaded",Initialize)


