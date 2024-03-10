import {ApiUsuarios} from "../api/ApiUsuarios.js"


let Usuarios = [];
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
                <td class="text-light">${item.nome}</td>
                <td class="text-light">${item.CPF}</td>
                <td class="d-flex align-items-center gap-4" colspan="3">
                    <button onclick="window.location.href='pag10.html'" class="btn-custom">Atualizar</button>
                    <button onclick="window.location.href='pag10.html'" class="btn btn-danger">Remover</button>
                </td>
            </tr>
            `
            )
    })
    contentPage.innerHTML += contentUsuarios.join("");
}

document.addEventListener("DOMContentLoaded",LoadUsuarios)
