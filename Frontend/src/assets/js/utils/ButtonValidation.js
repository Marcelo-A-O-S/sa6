export class ButtonValidation{
    button;
    constructor(_buttonid){
        this.button = document.getElementById(_buttonid);
    }
    disabledTrue(){
        this.button.disabled = true;
    }
    disabledFalse(){
        this.button.disabled = false;
    }
}
