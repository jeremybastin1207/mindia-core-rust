use std::error::Error;

use super::Task;

pub trait TaskStorage: Send + Sync {
    fn push(&self, task: Task) -> Result<(), Box<dyn Error>>;
    fn pop_queued(&self) -> Result<Option<Task>, Box<dyn Error>>;
}
