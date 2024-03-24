export const TYPEVALIDATION = {
    CPF: 'CPF',
    TEXT: 'text',
    EMAIL: 'email',
    TIME: 'time',
    DATE: 'date',
    NAME: 'name',
    NUMBER: 'number'
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
    setValue(value){
        const event = new Event("input");
        this.inputIdDOM.value = value;
        this.inputIdDOM.dispatchEvent(event)
    }
    async UpdateFields(){
        const event = new Event("input");
        this.inputIdDOM.dispatchEvent(event)
    }
    getValue(){
        return this.inputIdDOM.value;
    }
    onChangeInput(){
        const event = new Event("input");
        this.inputIdDOM.addEventListener("input",(e)=>{
            if(this.typeValidation == TYPEVALIDATION.NUMBER){

            }
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
                if(this.verifyFieldEmpty(e)){
                    this.valid = true;
                    this.isValid();
                }

            }
            if(this.typeValidation == TYPEVALIDATION.TIME){
                this.maskTime(e);
                if(this.verifyFieldEmpty(e)){
                    if(this.verifyTime(e)){
                        this.valid = true;
                        this.isValid()
                    }
                }
            }
            if(this.typeValidation == TYPEVALIDATION.DATE){
                if(this.verifyFieldEmpty(e)){
                    if(this.verifyDate(e)){
                        this.valid = true;
                        this.isValid();
                    }
                }
            }
        })
        this.inputIdDOM.dispatchEvent(event);
    }
    verifyFieldEmpty(e){
        if(e.target.value == "" && e.target.selectionEnd == 0){
            this.sucessIdDOM.innerHTML = "";
            this.erroIdDOM.innerHTML = "Preencha o campo acima!";
            return false;
        }else if(e.target.value !== ""){
            this.sucessIdDOM.innerHTML = "";
            this.erroIdDOM.innerHTML = "";
            return true;
        }

    }
    maskTime(e){
        if(e.target.selectionEnd == 2){
            e.target.value += ":"
        }else if(e.target.selectionEnd == 5){
            e.target.value += ":"
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
    verifyNumber(e){
        if(inputValue.search(/[a-zA-Z]/) > -1){
            let matchCaracteres = inputValue.match(/[a-zA-Z]/);
            this.erroIdDOM.innerHTML = `Os caracteres a seguir são inválidos: ${matchCaracteres.input}`;
            return false
        }
    }
    verifyDate(e){
        const dateRegex = /^[0-3][0-9]\/[0-1][0-9]\/[0-9][0-9][0-9][0-9]$/;
        let inputValue = e.target.value
        if(inputValue.search(/[a-zA-Z]/) > -1){
            let caracteresMatch = inputValue.match(/[a-zA-Z]/);
            this.sucessIdDOM.innerHTML = "";
            this.erroIdDOM.innerHTML = `Os caracteres a seguir são inválidos: ${caracteresMatch}`;
            return false
        }else if(dateRegex.test(inputValue)){
            return true
        }else{
            this.sucessIdDOM.innerHTML = "";
            this.erroIdDOM.innerHTML = `Formato de data inválido`;
            return false
        }
    }
}
