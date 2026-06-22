# 💎 TaskManager

*(Choose your language / Escolha seu idioma)*<br>
[![en](https://img.shields.io/badge/lang-en-red.svg)](https://github.com/HenriqueSHA/TaskManeger/blob/main/README.md)
[![pt-br](https://img.shields.io/badge/lang-pt--br-green.svg)](https://github.com/HenriqueSHA/TaskManeger/blob/main/README.pt-BR.md)

---

![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)
![License](https://img.shields.io/badge/License-MIT-blue.svg)

Aplicação interativa em linha de comando (CLI) desenvolvida em Rust para gerenciamento de tarefas pessoais, facilitando a criação, listagem, atualização de status e remoção de afazeres de forma ágil no terminal.

## 📋 Sumário
- [Sobre o Projeto](#-sobre-o-projeto)
- [Principais Funcionalidades](#-principais-funcionalidades)
- [Arquitetura e Tecnologias](#-arquitetura-e-tecnologias)
- [Instalação e Uso](#-instalação-e-uso)
- [Autor e Licença](#-autor-e-licenca)

---

## 📖 Sobre o Projeto
O TaskManager é uma ferramenta utilitária desenvolvida em Rust com o propósito de aprofundar os estudos em conceitos fundamentais da linguagem, como ownership, mutabilidade, gerenciamento de vetores dinâmicos (`Vec`), enums associados e leitura segura de dados via `std::io`. A aplicação opera em um loop interativo no terminal onde o usuário insere comandos numéricos para manipular tarefas em tempo de execução. O fluxo é simples: cada tarefa possui um identificador único auto-incrementado, uma descrição curta e um status que transita entre Pendente, Em Andamento e Concluído.

## ✨ Principais Funcionalidades
* **Adição Dinâmica de Tarefas:** Cria novos afazeres com geração automática de identificador único (ID) sequencial.
* **Listagem do Board de Tarefas:** Exibe todos os registros salvos em memória com seus respectivos IDs, descrições e status legíveis.
* **Atualização de Status Interativa:** Permite modificar o estado de uma tarefa específica (Pendente, Em Andamento, Concluído) localizando-a pelo seu ID.
* **Remoção de Tarefas:** Exclui tarefas do gerenciador a partir de seu ID correspondente com validação de existência.

## 💻 Arquitetura e Tecnologias
* **Core/Backend:** Rust (compilação via rustc e gerenciador Cargo)
* **Frontend:** Não aplicável (Interface CLI / Terminal)
* **Database/Ledger:** Armazenamento volátil em memória (Vector de structs)
* **DevOps:** Não aplicável

---

## 🚀 Instalação e Uso
Passo a passo para rodar localmente a aplicação no terminal.

### Pré-requisitos
* Rust (ferramenta `rustup` e gerenciador de pacotes `cargo`) v1.70+

### Execução Local (Sem Docker)
1. Clone o repositório e acesse a pasta do projeto:
   ```bash
   git clone git@github.com:HenriqueSHA/TaskManeger.git
   cd TaskManeger
   ```
2. Compile e execute o projeto diretamente com o Cargo:
   ```bash
   cargo run
   ```

### Exemplo de Uso
Ao rodar a aplicação, o seguinte menu interativo será exibido no terminal:
```text
--- Bem-vindo ao Task Manager ---
1 - Adicionar uma nova tarefa
2 - Listar todas as tarefas
3 - Atualizar o status de uma tarefa pelo ID
4 - Remover uma tarefa pelo ID
5 - Sair
---------------------------------
```
Exemplo de entrada/saída ao adicionar uma tarefa (Opção 1):
```text
Digite a descrição da tarefa:
Estudar Rust Ownership
Tarefa adicionada com sucesso! ID: 1
```

---

## 👨💻 Autor e Licença
Desenvolvido por **[Henrique Albergaria Santos](https://www.linkedin.com/in/henriquealbergaria/)**.
Distribuído sob a licença MIT. Veja `LICENSE` para mais informações.
