# TaskManager

*(Choose your language / Escolha seu idioma)*<br>
[![en](https://img.shields.io/badge/lang-en-red.svg)](https://github.com/HenriqueSHA/TaskManeger/blob/main/README.md)
[![pt-br](https://img.shields.io/badge/lang-pt--br-green.svg)](https://github.com/HenriqueSHA/TaskManeger/blob/main/README.pt-BR.md)

---

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/HenriqueSHA/TaskManeger)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/HenriqueSHA/TaskManeger/blob/main/LICENSE)

An interactive command-line interface (CLI) application written in Rust for managing personal tasks. It provides a simple, quick way to create, list, update, and remove tasks directly from the terminal.

## Table of Contents
- [About the Project](#about-the-project)
- [Key Features](#key-features)
- [Architecture and Technologies](#architecture-and-technologies)
- [Installation and Usage](#installation-and-usage)
- [Author and License](#author-and-license)

---

## About the Project
TaskManager is a command-line tool built in Rust to explore and practice core concepts of the language, such as ownership, mutability, dynamic array allocation (`Vec`), status enums, and console input/output parsing (`std::io`). 

The application runs inside an interactive loop in the terminal, processing user input options to manipulate task entities in memory. Each task contains an auto-incremented unique ID, a text description, and a state lifecycle transitioning between Pending, In Progress, and Completed.

## Key Features
* **Dynamic Task Creation:** Adds new tasks with an automatically generated sequential unique ID.
* **Task Board Listing:** Displays all tasks stored in memory alongside their IDs, text descriptions, and human-readable statuses.
* **Interactive Status Transition:** Allows changing the status of a specific task (Pending, In Progress, Completed) by locating it via its ID.
* **Task Removal:** Removes a task from the list using its ID with built-in existence validation.

## Architecture and Technologies

| Layer / Component | Technology | Description / Usage |
| :--- | :--- | :--- |
| **Core / Backend** | Rust | Compiled via `rustc` and managed by Cargo |
| **Interface (Frontend)** | CLI | Interactive terminal-based command-line interface |
| **Storage (Database)** | In-memory | Volatile storage utilizing a Rust Vector (`Vec<Struct>`) |

---

## Installation and Usage
Instructions on how to compile and run the application locally.

### Prerequisites
* Rust toolchain (rustup and cargo package manager) v1.70+

### Local Execution (Without Docker)
1. Clone the repository and navigate into the project directory:
   ```bash
   git clone git@github.com:HenriqueSHA/TaskManeger.git
   cd TaskManeger
   ```
2. Compile and run the application:
   ```bash
   cargo run
   ```

### Usage Example
When launching the application, you will be presented with the following interactive menu:
```text
--- Bem-vindo ao Task Manager ---
1 - Adicionar uma nova tarefa
2 - Listar todas as tarefas
3 - Atualizar o status de uma tarefa pelo ID
4 - Remover uma tarefa pelo ID
5 - Sair
---------------------------------
```
Example of adding a new task (Option 1):
```text
Digite a descrição da tarefa:
Estudar Rust Ownership
Tarefa adicionada com sucesso! ID: 1
```

---

## Author and License
Developed by **[Henrique Albergaria Santos](https://www.linkedin.com/in/henriquealbergaria/)**.

Distributed under the MIT License. See `LICENSE` for more information.
