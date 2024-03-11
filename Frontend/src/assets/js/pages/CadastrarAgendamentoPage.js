import {InputValidation,TYPEVALIDATION} from "../utils/InputValidation.js"
import {ApiAgendamento} from "../api/ApiAgendamento.js"
import {FormValidation} from "../utils/FormValidation.js"
import {ButtonValidation} from "../utils/ButtonValidation.js"
import {Agendamento} from "../models/Agendamento.js"
const Meses = [
    'Janeiro', 'Fevereiro', 'Março', 'Abril', 'Maio', 'Junho', 'Julho', 'Agosto', 'Setembro', 'Outubro', 'Novembro', 'Dezembro'
]


let btnLastMonth = document.getElementById("pre-year");
let btnNextMonth = document.getElementById("next-year");
const inputNome = new InputValidation("nome","nomeError","nomeSucess",TYPEVALIDATION.NAME);
const inputData = new InputValidation("Data","DataError","DataSucess",TYPEVALIDATION.DATE);
const inputHorarioInicial = new InputValidation("HorarioInicial", "HorarioInicialError","HorarioInicialSucess",TYPEVALIDATION.TIME)
const inputHorarioFinal = new InputValidation("HorarioFinal", "HorarioFinalError","HorarioFinalSucess",TYPEVALIDATION.TIME)
const btnSubmit = new ButtonValidation("btnSubmitAgendamento")
const FormSubmit = new FormValidation("formAgendamentos",[inputNome,inputData,inputHorarioInicial,inputHorarioFinal],btnSubmit)
FormSubmit.onChanges();
FormSubmit.onSubmit(()=>{

})

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
}

document.addEventListener("DOMContentLoaded",Initialize)


