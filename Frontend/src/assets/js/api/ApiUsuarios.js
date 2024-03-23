import { Usuario } from "../models/Usuario.js"

export class ApiUsuarios {
    async getUsuarios() {
        let Usuarios = await fetch("http://localhost:8080/Usuarios", {
            method: "GET"
        })
            .then((response) => {
                return response.json()
            });
        return Usuarios;
    }
    async createUsuarios(usuario) {
        console.log(JSON.stringify(usuario))
        return await fetch("http://localhost:8080/CreateUsuario", {
            body: JSON.stringify(usuario),
            method: 'POST',
            mode: "cors",
            headers: {
                'Content-Type': 'application/json'
            }
        }).then((response) => response.json())
            .then((data) => { 
                return data
             })
            .catch((err) => {
                return err
            })
    }
    async getUsuarioById(Id){
        return await fetch(`http://localhost:8080/UsuarioGetById/${Id}`, {
            method: "GET"
        })
        .then((response) =>  response.json())
        .then((data) => { return data});
            
    }
    async deleteUsuarioById(Id){
        return await fetch(`http://localhost:8080/DeleteById/${Id}`, {
            method: "DELETE"
        })
        .then((response) =>  response.json())
        .then((data) => { return data});
    }
}
