import { InputValidation, TYPEVALIDATION } from "../utils/InputValidation.js"
import { ButtonValidation } from "../utils/ButtonValidation.js"
import { FormValidation } from "../utils/FormValidation.js"
import { Usuario } from "../models/Usuario.js"


const inputNome = new InputValidation("nome","nomeError", "nomeSucess",TYPEVALIDATION.NAME);
const inputCPF = new InputValidation("CPF","CPFError", "CPFSucess", TYPEVALIDATION.CPF);
const btnSubmit = new ButtonValidation("btnSubmitUsuarios");
const formSubmit = new FormValidation("formUsuarios",[inputNome,inputCPF], btnSubmit)
formSubmit.onChanges();
formSubmit.onSubmit(()=>{
    const usuario = new Usuario();
    usuario.CPF = inputCPF.getValue();
    usuario.nome = inputNome.getValue();
    console.log(usuario)

})
