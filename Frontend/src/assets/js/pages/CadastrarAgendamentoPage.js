import {InputValidation,TYPEVALIDATION} from "../utils/InputValidation.js"
import {ApiAgendamento} from "../api/ApiAgendamento.js"
let day = document.querySelectorAll(".day")
let year = document.getElementById("year")
const inputNome = new InputValidation("nome","nomeError","nomeSucess",TYPEVALIDATION.NAME);
const inputDia = new InputValidation("Dia","DiaError","DiaSucess",TYPEVALIDATION.DATE);
let testeData = new Date();

async function Initialize(){
    const apiAgendamentos = new ApiAgendamento();
    let agendamentos = await apiAgendamentos.getAgendamentos();
    day.forEach((item)=>{
        item.addEventListener("click",(e)=>{
            console.log("Fui clickado")

        })
        agendamentos.map((agendamento)=>{
            if(item.innerHTML == agendamento.Dia){
                item.classList.add("block")
            }
        })
    })
}

document.addEventListener("DOMContentLoaded",Initialize)


