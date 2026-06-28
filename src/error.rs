use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Task with ID {0} not found.")]
    TaskNotFound(u32),

    #[error("I/O error occurred: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization/deserialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
