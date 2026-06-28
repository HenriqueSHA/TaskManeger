use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Pendente,
    EmAndamento,
    Concluido,
}

impl Status {
    pub fn descricao(&self) -> &str {
        match self {
            Status::Pendente => "Pendente",
            Status::EmAndamento => "Em Andamento",
            Status::Concluido => "Concluído",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub descricao: String,
    pub status: Status,
    pub criada_em: DateTime<Local>,
    pub atualizada_em: DateTime<Local>,
}

impl Task {
    pub fn new(id: u32, descricao: String) -> Self {
        let agora = Local::now();
        Task {
            id,
            descricao,
            status: Status::Pendente,
            criada_em: agora,
            atualizada_em: agora,
        }
    }

    pub fn atualizar_status(&mut self, novo_status: Status) {
        self.status = novo_status;
        self.atualizada_em = Local::now();
    }
}
