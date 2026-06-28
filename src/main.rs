use colored::Colorize;
use inquire::{error::InquireError, Confirm, Select, Text};
use std::fmt;
use task_maneger::{JsonFileRepository, Status, Task, TaskRepository};

struct TaskOption(Task);

impl fmt::Display for TaskOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_desc = match self.0.status {
            Status::Pendente => "Pendente".red().bold(),
            Status::EmAndamento => "Em Andamento".yellow().bold(),
            Status::Concluido => "Concluído".green().bold(),
        };
        write!(
            f,
            "ID: {} | [{}] {}",
            self.0.id, status_desc, self.0.descricao
        )
    }
}

fn main() {
    println!("{}", "=== TASK MANAGER CLI ===".cyan().bold());

    let mut repo = match JsonFileRepository::new("tasks.json") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}: {}", "Erro ao iniciar repositório".red().bold(), e);
            std::process::exit(1);
        }
    };

    loop {
        let options = vec![
            "1. Adicionar uma nova tarefa",
            "2. Listar todas as tarefas",
            "3. Atualizar o status de uma tarefa",
            "4. Remover uma tarefa",
            "5. Sair",
        ];

        let ans = Select::new("Escolha uma opção:", options).prompt();

        match ans {
            Ok("1. Adicionar uma nova tarefa") => {
                if let Err(e) = adicionar_tarefa(&mut repo) {
                    eprintln!("{}: {}", "Erro".red().bold(), e);
                }
            }
            Ok("2. Listar todas as tarefas") => {
                if let Err(e) = listar_tarefas(&repo) {
                    eprintln!("{}: {}", "Erro".red().bold(), e);
                }
            }
            Ok("3. Atualizar o status de uma tarefa") => {
                if let Err(e) = atualizar_tarefa(&mut repo) {
                    eprintln!("{}: {}", "Erro".red().bold(), e);
                }
            }
            Ok("4. Remover uma tarefa") => {
                if let Err(e) = remover_tarefa(&mut repo) {
                    eprintln!("{}: {}", "Erro".red().bold(), e);
                }
            }
            Ok("5. Sair") => {
                println!("{}", "Saindo... Até mais!".cyan());
                break;
            }
            Err(InquireError::OperationCanceled) | Err(InquireError::OperationInterrupted) => {
                println!("\nSaindo... Até mais!");
                break;
            }
            _ => {
                println!("{}", "Opção inválida.".red());
            }
        }
    }
}

fn adicionar_tarefa(repo: &mut JsonFileRepository) -> Result<(), InquireError> {
    let descricao = Text::new("Digite a descrição da tarefa:").prompt()?;
    let descricao_trimmed = descricao.trim();
    if descricao_trimmed.is_empty() {
        println!("{}", "A descrição não pode ser vazia.".red());
        return Ok(());
    }

    match repo.add(descricao_trimmed.to_string()) {
        Ok(t) => println!(
            "{}: ID {} - {}",
            "Tarefa adicionada com sucesso".green().bold(),
            t.id,
            t.descricao
        ),
        Err(e) => eprintln!("Erro ao salvar no arquivo: {}", e),
    }

    Ok(())
}

fn listar_tarefas(repo: &JsonFileRepository) -> Result<(), InquireError> {
    let tasks = match repo.list() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Erro ao carregar tarefas: {}", e);
            return Ok(());
        }
    };

    if tasks.is_empty() {
        println!("{}", "Nenhuma tarefa cadastrada.".yellow());
        return Ok(());
    }

    println!("\n{}", "--- Lista de Tarefas ---".cyan().bold());
    println!(
        "{:<5} | {:<15} | {:<30} | {:<20}",
        "ID", "Status", "Descrição", "Criada em"
    );
    println!("{}", "-".repeat(78).dimmed());

    for task in tasks {
        let status_colored = match task.status {
            Status::Pendente => "Pendente".red().bold(),
            Status::EmAndamento => "Em Andamento".yellow().bold(),
            Status::Concluido => "Concluído".green().bold(),
        };

        let data_formatada = task.criada_em.format("%Y-%m-%d %H:%M:%S").to_string();

        println!(
            "{:<5} | {:<24} | {:<30} | {:<20}",
            task.id,
            status_colored,
            task.descricao,
            data_formatada.dimmed()
        );
    }
    println!();

    Ok(())
}

fn atualizar_tarefa(repo: &mut JsonFileRepository) -> Result<(), InquireError> {
    let tasks = match repo.list() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Erro ao carregar tarefas: {}", e);
            return Ok(());
        }
    };

    if tasks.is_empty() {
        println!("{}", "Nenhuma tarefa cadastrada para atualizar.".yellow());
        return Ok(());
    }

    let task_options: Vec<TaskOption> = tasks.into_iter().map(TaskOption).collect();
    let selected_task =
        Select::new("Selecione a tarefa para atualizar o status:", task_options).prompt()?;

    let status_options = vec!["Pendente", "Em Andamento", "Concluído"];
    let selected_status = Select::new("Escolha o novo status:", status_options).prompt()?;

    let novo_status = match selected_status {
        "Pendente" => Status::Pendente,
        "Em Andamento" => Status::EmAndamento,
        "Concluído" => Status::Concluido,
        _ => unreachable!(),
    };

    match repo.update_status(selected_task.0.id, novo_status) {
        Ok(_) => println!("{}", "Status atualizado com sucesso!".green().bold()),
        Err(e) => eprintln!("Erro ao atualizar status: {}", e),
    }

    Ok(())
}

fn remover_tarefa(repo: &mut JsonFileRepository) -> Result<(), InquireError> {
    let tasks = match repo.list() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Erro ao carregar tarefas: {}", e);
            return Ok(());
        }
    };

    if tasks.is_empty() {
        println!("{}", "Nenhuma tarefa cadastrada para remover.".yellow());
        return Ok(());
    }

    let task_options: Vec<TaskOption> = tasks.into_iter().map(TaskOption).collect();
    let selected_task = Select::new("Selecione a tarefa para remover:", task_options).prompt()?;

    let confirmar = Confirm::new("Tem certeza que deseja remover esta tarefa?")
        .with_default(false)
        .prompt()?;

    if confirmar {
        match repo.remove(selected_task.0.id) {
            Ok(_) => println!("{}", "Tarefa removida com sucesso!".green().bold()),
            Err(e) => eprintln!("Erro ao remover tarefa: {}", e),
        }
    } else {
        println!("{}", "Remoção cancelada.".yellow());
    }

    Ok(())
}
