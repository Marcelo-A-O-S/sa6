import { ButtonValidation } from "./ButtonValidation.js";

export class FormValidation{
    form;
    inputs = [];
    button = new ButtonValidation();
    constructor(_formId,_inputs, _button){
        this.form = document.getElementById(_formId);
        this.inputs = _inputs;
        this.button = _button;
        this.form.addEventListener('input', () => {
            this.verifyValidation();
        });
    }
    verifyValidation(){
        let valores = this.inputs.every(x=> x.valid);
        console.log(valores);
        if(valores){
            this.button.disabledFalse();
        }else{
            this.button.disabledTrue();
        }
    }
    onSubmit(functionSubmit){
        this.form.addEventListener('submit',(e)=>{
            e.preventDefault();
            functionSubmit();
        })
    }
    onChanges(){
        this.inputs.map((item)=>{
            item.onChangeInput();
        })
    }
}
