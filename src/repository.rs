use crate::error::TaskError;
use crate::task::{Status, Task};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub trait TaskRepository {
    fn add(&mut self, descricao: String) -> Result<Task, TaskError>;
    fn list(&self) -> Result<Vec<Task>, TaskError>;
    fn find_by_id(&self, id: u32) -> Result<Task, TaskError>;
    fn update_status(&mut self, id: u32, novo_status: Status) -> Result<Task, TaskError>;
    fn remove(&mut self, id: u32) -> Result<(), TaskError>;
}

pub struct JsonFileRepository {
    file_path: PathBuf,
    tasks: Vec<Task>,
    next_id: u32,
}

impl JsonFileRepository {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Result<Self, TaskError> {
        let path = file_path.as_ref().to_path_buf();
        let mut tasks: Vec<Task> = Vec::new();
        let mut next_id = 1;

        if path.exists() {
            let mut file = File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            if !contents.trim().is_empty() {
                tasks = serde_json::from_str(&contents)?;
                if let Some(max_task) = tasks.iter().max_by_key(|t| t.id) {
                    next_id = max_task.id + 1;
                }
            }
        }

        Ok(JsonFileRepository {
            file_path: path,
            tasks,
            next_id,
        })
    }

    fn save(&self) -> Result<(), TaskError> {
        let serialized = serde_json::to_string_pretty(&self.tasks)?;
        let mut file = File::create(&self.file_path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}

impl TaskRepository for JsonFileRepository {
    fn add(&mut self, descricao: String) -> Result<Task, TaskError> {
        let new_task = Task::new(self.next_id, descricao);
        self.next_id += 1;
        self.tasks.push(new_task.clone());
        self.save()?;
        Ok(new_task)
    }

    fn list(&self) -> Result<Vec<Task>, TaskError> {
        Ok(self.tasks.clone())
    }

    fn find_by_id(&self, id: u32) -> Result<Task, TaskError> {
        self.tasks
            .iter()
            .find(|t| t.id == id)
            .cloned()
            .ok_or(TaskError::TaskNotFound(id))
    }

    fn update_status(&mut self, id: u32, novo_status: Status) -> Result<Task, TaskError> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.atualizar_status(novo_status);
            let updated_task = task.clone();
            self.save()?;
            Ok(updated_task)
        } else {
            Err(TaskError::TaskNotFound(id))
        }
    }

    fn remove(&mut self, id: u32) -> Result<(), TaskError> {
        let position = self
            .tasks
            .iter()
            .position(|t| t.id == id)
            .ok_or(TaskError::TaskNotFound(id))?;

        self.tasks.remove(position);
        self.save()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestContext {
        path: PathBuf,
    }

    impl TestContext {
        fn new(name: &str) -> Self {
            let path = PathBuf::from(format!("test_tasks_{}.json", name));
            let _ = std::fs::remove_file(&path);
            Self { path }
        }
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.path);
        }
    }

    #[test]
    fn test_add_and_list_tasks() {
        let context = TestContext::new("add_list");
        let mut repo = JsonFileRepository::new(&context.path).unwrap();

        assert!(repo.list().unwrap().is_empty());

        let t1 = repo.add("Primeira tarefa".to_string()).unwrap();
        assert_eq!(t1.id, 1);
        assert_eq!(t1.descricao, "Primeira tarefa");
        assert_eq!(t1.status, Status::Pendente);

        let t2 = repo.add("Segunda tarefa".to_string()).unwrap();
        assert_eq!(t2.id, 2);

        let tasks = repo.list().unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].descricao, "Primeira tarefa");
        assert_eq!(tasks[1].descricao, "Segunda tarefa");
    }

    #[test]
    fn test_update_status() {
        let context = TestContext::new("update_status");
        let mut repo = JsonFileRepository::new(&context.path).unwrap();

        let t = repo.add("Testar status".to_string()).unwrap();
        assert_eq!(t.status, Status::Pendente);

        let updated = repo.update_status(t.id, Status::EmAndamento).unwrap();
        assert_eq!(updated.status, Status::EmAndamento);

        let fetched = repo.find_by_id(t.id).unwrap();
        assert_eq!(fetched.status, Status::EmAndamento);
    }

    #[test]
    fn test_remove_task() {
        let context = TestContext::new("remove_task");
        let mut repo = JsonFileRepository::new(&context.path).unwrap();

        let t = repo.add("Remover-me".to_string()).unwrap();
        assert_eq!(repo.list().unwrap().len(), 1);

        repo.remove(t.id).unwrap();
        assert_eq!(repo.list().unwrap().len(), 0);

        let result = repo.find_by_id(t.id);
        assert!(result.is_err());
    }
}
