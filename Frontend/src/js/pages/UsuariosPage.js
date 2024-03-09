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
            <tr>
                <td class="text-light">${item.nome}</td>
                <td class="text-light">${item.CPF}</td>
                <td colspan="3">
                    <button onclick="window.location.href='pag10.html'" class="btn btn-primary">Atualizar</button>
                    <button onclick="window.location.href='pag10.html'" class="btn btn-danger">Remover</button>
                </td>
            </tr>
            `
            )
    })
    contentPage.innerHTML += contentUsuarios.join("");
}

document.addEventListener("DOMContentLoaded",LoadUsuarios)
