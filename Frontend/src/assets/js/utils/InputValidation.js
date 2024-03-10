export const TYPEVALIDATION = {
    CPF: 'CPF',
    TEXT: 'text',
    EMAIL: 'email',
    TIME: 'time',
    DATE: 'date',
    NAME: 'name'
  };

export class InputValidation{
    inputIdDOM;
    erroIdDOM;
    sucessIdDOM;
    typeValidation;
    valid;
    constructor(_inputId, _erroId, _sucessId, _typeValidation){
        this.inputIdDOM = document.getElementById(_inputId);
        this.erroIdDOM = document.getElementById(_erroId);
        this.sucessIdDOM = document.getElementById(_sucessId);
        this.typeValidation = _typeValidation;
        this.valid = false;
    }
    getValue(){
        return this.inputIdDOM.value;
    }
    onChangeInput(){
        this.inputIdDOM.addEventListener("input",(e)=>{
            if(this.typeValidation == TYPEVALIDATION.NAME){
                if(this.verifyFieldEmpty(e)){
                    if(this.verifyName(e)){
                        this.valid = true;
                        this.isValid()
                    }
                }

            }
            if(this.typeValidation == TYPEVALIDATION.TEXT){
                if(this.verifyFieldEmpty(e)){
                    this.valid = true;
                    this.isValid()
                }

            }
            if(this.typeValidation == TYPEVALIDATION.CPF){
                this.maskCPF(e)
                if(this.verifyFieldEmpty(e)){
                    if(this.verifyValidationCpF(e)){
                        this.valid = true;
                        this.isValid()
                    }
                }

            }
            if(this.typeValidation == TYPEVALIDATION.EMAIL){
                this.verifyFieldEmpty(e);
                this.isValid()
            }
            if(this.typeValidation == TYPEVALIDATION.TIME){
                this.verifyFieldEmpty(e);
                this.verifyTime(e);
                this.isValid()
            }
        })
    }
    verifyFieldEmpty(e){
        if(e.target.value == ""){
            this.sucessIdDOM.innerHTML = "";
            this.erroIdDOM.innerHTML = "Preencha o campo acima!";
            return false;
        }else if(e.target.value !== ""){
            this.sucessIdDOM.innerHTML = "";
            this.erroIdDOM.innerHTML = "";
            return true;
        }

    }
    maskCPF(e){
        if(e.target.selectionEnd == 3){
            e.target.value += "."
        }else if(e.target.selectionEnd == 7){
            e.target.value += "."
        }else if(e.target.selectionEnd == 11){
            e.target.value += "-"
        }
    }
    verifyValidationCpF(e){
        const cpfRegex = /^[0-9]+\.[0-9]+\.[0-9]+-[0-9][0-9]$/;
        let inputValue = "";
        inputValue = e.target.value;
        if(inputValue.search(/[a-zA-Z]/) > -1){
            let matchCaracteres = inputValue.match(/[a-zA-Z]/);
            this.erroIdDOM.innerHTML = `Os caracteres a seguir são inválidos: ${matchCaracteres.input}`;
            return false
        }else if(cpfRegex.test(e.target.value)){
            console.log("CPF válido");
            return true
        }else{
            this.sucessIdDOM.innerHTML = "";
            this.erroIdDOM.innerHTML = "Formato inválido ou incompleto";
            return false
        }
    }
    verifyTime(e){
        const timeRegex = /^(?:[01]?[0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$/;
        if(timeRegex.test(e.target.value)){
            console.log("Hora válida");
            return true
        }else{
            this.sucessIdDOM.innerHTML = "";
            this.erroIdDOM.innerHTML = "Hora informada inválida";
            return false
        }
    }
    isValid(){
        if(this.valid === true){
            this.erroIdDOM.innerHTML = "";
            this.sucessIdDOM.innerHTML = "Pode prosseguir";

        }
    }
    verifyName(e){
        let inputValue = "";
        inputValue = e.target.value;
        if(inputValue.search(/[0-9]/) > -1){
            let matchCaracteres = inputValue.match(/[0-9]/);
            console.log(matchCaracteres)
            this.sucessIdDOM.innerHTML = "";
            this.erroIdDOM.innerHTML = `Os caracteres a seguir são inválidos: ${matchCaracteres.input}`;
            return false
        }else{
            return true
        }
    }
}
