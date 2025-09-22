# 🏋️‍♂️ SA6 – Golden House

## 📌 Etapa 1: Agendamento de Horários de Academias

### 👋 Introdução

Seja bem-vindo à **SA6**, ou melhor, à **Golden House**.  
Uma aplicação desenvolvida pelos seguintes integrantes:

- **Marcelo A.** – Product Owner  
- **Henrique D.** – Front-end  
- **Samuel M.** – Back-end  
- **Rafael H.** – Banco de Dados  
- **Gabriel I. do Brasil** – Front-end  

O projeto foi planejado com base no **modelo SCRUM** de gerenciamento ágil, realizando **reuniões diárias** ao final do dia para revisar o que foi desenvolvido e definir os próximos passos.

A aplicação possui entre **9 e 10 páginas**, com uso intensivo de **APIs** como pilar central do projeto. Isso nos permitiu:
- Melhorar a performance e reduzir a poluição visual do sistema.  
- Reaproveitar código de forma inteligente.  
- Aumentar a produtividade dos desenvolvedores.  
- Estimular a inovação ao integrar dados e funcionalidades de outros sistemas.  

Além disso, adotamos o **Modelo de Divisão Funcional de Bibliotecas**, o que trouxe benefícios como:
- Desenvolvimento mais rápido e inteligente.  
- Melhor visualização de comandos e chamadas do sistema.  
- Facilidade na manutenção, permitindo alterações pontuais sem comprometer o restante do código.  
- Redução de riscos de regressões e efeitos colaterais.  

### 🚀 Tecnologia Utilizada

A linguagem escolhida para o **back-end** foi o **Rust**, que vem crescendo de forma exponencial e é considerada por muitos como a **linguagem do futuro**. Desenvolvida pela Mozilla Research, o Rust foi projetado para ser **seguro, concorrente e prático**.  

Entre suas principais vantagens destacamos:
- **Segurança de memória em tempo de compilação**, sem necessidade de garbage collector.  
- **Portabilidade**, com suporte a compilação cruzada para múltiplas plataformas e arquiteturas.  
- **Alto desempenho**, ideal para aplicações críticas e escaláveis.  

Essas características atenderam plenamente às nossas expectativas para o desenvolvimento do back-end da Golden House.  

---

## 📑 Especificações – Etapa 1

- **Desenvolvedores**: Henrique D., Marcelo A., Samuel M., Rafael H., Gabriel I. do Brasil  
- **Linguagens de marcação e estilo**: HTML, CSS  
- **Linguagens de programação**: JavaScript, Rust  
- **Banco de Dados**: MySQL Workbench  
- **Modelo de Gerenciamento do Projeto**: SCRUM  

## 📦 Dependências do Projeto

Este projeto utiliza as seguintes bibliotecas Rust:

- **[chrono](https://crates.io/crates/chrono) `0.4.34`**  
  Biblioteca para manipulação de datas e horários.  
  - Features habilitadas: `serde` (para serialização/deserialização de datas).

- **[actix-web](https://crates.io/crates/actix-web) `4`**  
  Framework para construção de servidores web assíncronos e APIs REST.

- **[diesel](https://crates.io/crates/diesel) `2.1.0`**  
  ORM para Rust, facilita interação com bancos de dados relacionais.  
  - Features habilitadas: `mysql`, `chrono`.

- **[dotenvy](https://crates.io/crates/dotenvy) `0.15`**  
  Carrega variáveis de ambiente a partir de arquivos `.env`.

- **[serde](https://crates.io/crates/serde) `1.0.197`**  
  Framework para serialização e deserialização de dados em vários formatos (ex: JSON).

- **[mysql](https://crates.io/crates/mysql) `24.0.0`**  
  Cliente para conexão direta com bancos de dados MySQL.

- **[actix-rt](https://crates.io/crates/actix-rt) `2.9.0`**  
  Runtime assíncrono utilizado pelo Actix para execução de tarefas.

- **[actix-cors](https://crates.io/crates/actix-cors) `0.7.0`**  
  Middleware para habilitar e configurar **CORS** em aplicações Actix-Web.

Front-end: [text](https://github.com/192el/sa6/tree/main/Frontend)

Back-end: [text](https://github.com/192el/sa6/tree/main/Backend)

Banco de Dados:[text](https://github.com/192el/sa6/tree/main/Database)

Link do Vídeo:  






