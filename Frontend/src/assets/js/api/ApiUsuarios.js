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
        await fetch("http://localhost:8080/CreateUsuario", {
            body: JSON.stringify(usuario),
            method: 'POST',
            mode: "no-cors",
            headers: {
                'Content-Type': 'application/json'
            }
        }).then((response) => response.json())
            .then((data) => { console.log(data) })
            .catch((err) => {
                console.log(err)
            })
    }
}
