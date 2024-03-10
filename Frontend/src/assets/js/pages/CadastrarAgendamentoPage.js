import {InputValidation,TYPEVALIDATION} from "../utils/InputValidation.js"
import {ApiAgendamento} from "../api/ApiAgendamento.js"
import {FormValidation} from "../utils/FormValidation.js"
import {ButtonValidation} from "../utils/ButtonValidation.js"

let year = document.getElementById("year");
let dateCurrent = document.querySelector(".current-day");
let btnLastMonth = document.getElementById("pre-year");
let btnNextMonth = document.getElementById("next-year");
const inputNome = new InputValidation("nome","nomeError","nomeSucess",TYPEVALIDATION.NAME);
const inputData = new InputValidation("Data","DataError","DataSucess",TYPEVALIDATION.DATE);
const inputHorarioInicial = new InputValidation("HorarioInicial", "HorarioInicialError","HorarioInicialSucess",TYPEVALIDATION.TIME)
const inputHorarioFinal = new InputValidation("HorarioFinal", "HorarioFinalError","HorarioFinalSucess",TYPEVALIDATION.TIME)
const btnSubmit = new ButtonValidation("btnSubmitAgendamento")
const FormSubmit = new FormValidation("formAgendamentos",[inputNome,inputData,inputHorarioInicial,inputHorarioFinal],btnSubmit)
FormSubmit.onChanges();
async function InsertOnClick(){
    let day = document.querySelectorAll(".day");
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
}

document.addEventListener("DOMContentLoaded",Initialize)


