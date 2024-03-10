export const TYPEVALIDATION = {
    CPF: 'CPF',
    TEXT: 'text',
    EMAIL: 'email',
    TIME: 'time',
    DATE: 'date'
  };

export class InputValidation{
    inputIdDOM;
    erroIdDOM;
    sucessIdDOM;
    typeValidation;
    constructor(_inputId, _erroId, _sucessId, _typeValidation){
        this.inputIdDOM = document.getElementById(_inputId);
        this.erroIdDOM = document.getElementById(_erroId);
        this.sucessIdDOM = document.getElementById(_sucessId);
        this.typeValidation = _typeValidation;
    }
    onChangeInput(){
        this.inputIdDOM.addEventListener("input",(e)=>{
            if(this.typeValidation == TYPEVALIDATION.TEXT){
                this.verifyFieldEmpty(e);
            }
            if(this.typeValidation == TYPEVALIDATION.CPF){
                console.log(e)
                this.maskCPF(e)
                this.verifyFieldEmpty(e);
                this.verifyValidationCpF(e);
            }
            if(this.typeValidation == TYPEVALIDATION.EMAIL){
                this.verifyFieldEmpty(e);
            }
            if(this.typeValidation == TYPEVALIDATION.TIME){
                this.verifyFieldEmpty(e);
                this.verifyTime(e);
            }
        })
    }
    verifyFieldEmpty(e){
        if(e.target.value == ""){
            this.sucessIdDOM.innerHTML = "";
            this.erroIdDOM.innerHTML = "Preencha o campo acima!";
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
        const cpfRegex = /^[0-9]+\.[0-9]+\.[0-9]+-[0-9]+$/;
                if(cpfRegex.test(e.target.value)){
                    console.log("CPF válido");
                }else{
                    this.sucessIdDOM.innerHTML = "";
                    this.erroIdDOM.innerHTML = "CPF só utiliza caracteres numericos, corrija para prosseguir!";
                }
    }
    verifyTime(e){
        const timeRegex = /^(?:[01]?[0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$/;
                if(timeRegex.test(e.target.value)){
                    console.log("CPF válido");
                }else{
                    this.sucessIdDOM.innerHTML = "";
                    this.erroIdDOM.innerHTML = "Hora informada inválida";
                }
    }
}
