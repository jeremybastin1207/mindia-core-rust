use redis::Connection;
use std::error::Error;
use std::sync::{Arc, Mutex};

use super::{Task, TaskStatus, TaskStorage};

const QUEUED_TASKS_QUEUE_KEY: &str = "internal:queue:tasks:queued";
const COMPLETED_TASKS_QUEUE_KEY: &str = "internal:queue:tasks:completed";

pub struct RedisTaskStorage {
    conn: Arc<Mutex<Connection>>,
}

impl RedisTaskStorage {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

impl TaskStorage for RedisTaskStorage {
    fn push(&self, task: Task) -> Result<(), Box<dyn Error>> {
        let key = match task.status {
            TaskStatus::Queued => QUEUED_TASKS_QUEUE_KEY,
            TaskStatus::Completed => COMPLETED_TASKS_QUEUE_KEY,
        };

        redis::cmd("RPUSH")
            .arg(key)
            .arg(serde_json::to_string(&task)?)
            .query(&mut self.conn.lock().unwrap())?;

        Ok(())
    }

    fn pop_queued(&self) -> Result<Option<Task>, Box<dyn Error>> {
        let result: Option<String> = redis::cmd("LPOP")
            .arg(QUEUED_TASKS_QUEUE_KEY)
            .query(&mut self.conn.lock().unwrap())?;

        match result {
            Some(json) => {
                let task: Task = serde_json::from_str(&json)?;
                Ok(Some(task))
            }
            None => Ok(None),
        }
    }
}
