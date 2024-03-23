import {ApiUsuarios} from "../api/ApiUsuarios.js"
import { Usuario } from "../models/Usuario.js";

async function Deletar(Id){
    console.log("Executando deleção ...")
    const apiUsuarios = new ApiUsuarios()
    let response = apiUsuarios.deleteUsuarioById(Id);
    window.location.href=""
}
let buttons = document.querySelectorAll("#btnDelete")
buttons.forEach((item)=>{
    console.log(item.innerHTML)
    item.addEventListener("click",(e)=>{
        console.log(item)
        console.log(e);
    })
})
async function teste(){
    console.log("teste")
}
console.log(buttons)
async function loadButtonsDelete(){
    console.log("Carregando botões")
    let buttons = document.querySelectorAll("#btnDelete")
    buttons.forEach((item)=>{
        item.addEventListener("click",(e)=>{
            console.log("teste")
            console.log(e);
        })
    })
}
document.addEventListener('click', function(event) {
    if (event.target.matches('#btnDelete')) {
      console.log(event.target.dataset.id)
      const apiUsuarios = new ApiUsuarios();
      apiUsuarios.deleteUsuarioById(event.target.dataset.id)
      window.location.href=""
    }
  });
let Usuarios = new Array();
let contentUsuarios;
async function LoadUsuarios(){
    const contentPage = document.getElementById("content-usuarios");
    const apiUsuarios = new ApiUsuarios();
    Usuarios = await apiUsuarios.getUsuarios();
    console.log(Usuarios)
    contentUsuarios = Usuarios.map(item=>{
        return(
            `
            <tr class="">
            <td class="text-light">${item.Id}</td>
                <td class="text-light">${item.nome}</td>
                <td class="text-light">${item.CPF}</td>
                <td class="d-flex align-items-center gap-4" colspan="3">
                    <button onclick="window.location.href='CriarUsuarios.html?Id=${item.Id}'" class="btn-custom">Atualizar</button>
                    <button data-id=${item.Id} Id="btnDelete"  class="btn btn-danger">Remover</button>
                </td>
            </tr>
            `
            )
    })
    contentPage.innerHTML += contentUsuarios.join("");
}

document.addEventListener("DOMContentLoaded",LoadUsuarios)
//document.addEventListener("DOMContentLoaded",loadButtonsDelete)
