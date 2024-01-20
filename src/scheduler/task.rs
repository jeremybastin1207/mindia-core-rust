use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait TaskExecutor: Send + Sync {
    async fn run(&self, task: Task) -> Result<Task, Box<dyn Error>>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Queued,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub enum TaskKind {
    ClearCache,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Details {
    ClearCache { before_date: DateTime<Utc> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub status: TaskStatus,
    pub details: Details,
    pub kind: TaskKind,
    pub error: Option<String>,
}

impl Task {
    pub fn new(kind: TaskKind, details: Details) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            status: TaskStatus::Queued,
            details,
            kind,
            error: None,
        }
    }
}
