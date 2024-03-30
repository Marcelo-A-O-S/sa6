import { InputValidation, TYPEVALIDATION } from "../utils/InputValidation.js"
import { ButtonValidation } from "../utils/ButtonValidation.js"
import { FormValidation } from "../utils/FormValidation.js"
import { Usuario } from "../models/Usuario.js"
import { ApiUsuarios } from "../api/ApiUsuarios.js"
const params = new URLSearchParams(window.location.search);
const usuarioId = params.get("Id");
const titlePage = document.getElementById("title-page")
const titleForm = document.getElementById("title-form")
if(usuarioId != null){
    titlePage.innerHTML = "Atualizar Usuários"
    titleForm.innerHTML = "Atualizar Usuários"
}
const inputId = new InputValidation("Id","IdError","IdSucess",TYPEVALIDATION.NUMBER)
const inputNome = new InputValidation("nome","nomeError", "nomeSucess",TYPEVALIDATION.NAME);
const inputCPF = new InputValidation("CPF","CPFError", "CPFSucess", TYPEVALIDATION.CPF);
const btnSubmit = new ButtonValidation("btnSubmitUsuarios");
const formSubmit = new FormValidation("formUsuarios",[inputNome,inputCPF], btnSubmit)
formSubmit.onChanges();

async function LoadUsuario(){
    if(usuarioId != null){
        let usuarios = [];
        let usuario;
        const apiUsuarios = new ApiUsuarios();
        usuario = await apiUsuarios.getUsuarioById(parseInt(usuarioId));
        console.log(usuario)
        inputId.setValue(usuario.Id)
        inputNome.setValue(usuario.nome);
        inputCPF.setValue(usuario.CPF);
    }else{
        inputId.setValue(0);
    }
}
formSubmit.onSubmit(async ()=>{
    let response;
    const apiUsuarios = new ApiUsuarios();
    const usuario = new Usuario();
    usuario.Id = parseInt(inputId.getValue());
    usuario.CPF = inputCPF.getValue();
    usuario.nome = inputNome.getValue();
    console.log(usuario);
    try{
        response = await apiUsuarios.createUsuarios(usuario);
        console.log("Response :", response);
        alert(response)
    }catch(err){
        console.log(err)
    }

})
document.addEventListener('DOMContentLoaded',LoadUsuario)

