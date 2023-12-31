use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use super::{Task, TaskExecutor, TaskKind, TaskStatus, TaskStorage, ThreadPool};

pub struct TaskScheduler {
    pub task_storage: Arc<dyn TaskStorage>,
    pub thread_pool: ThreadPool,
    pub task_executors: HashMap<TaskKind, Arc<dyn TaskExecutor>>,
}

impl TaskScheduler {
    pub fn new(task_storage: Arc<dyn TaskStorage>) -> Self {
        Self {
            task_storage,
            thread_pool: ThreadPool::new(4),
            task_executors: HashMap::new(),
        }
    }

    pub fn register_task_executor(
        &mut self,
        task_kind: TaskKind,
        task_executor: Arc<dyn TaskExecutor>,
    ) {
        self.task_executors.insert(task_kind, task_executor);
    }

    pub fn push(&self, task: Task) -> Result<(), Box<dyn Error>> {
        self.task_storage.push(task)
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        loop {
            match self.task_storage.pop_queued()? {
                Some(task) => {
                    let task_executor = self.task_executors.get(&task.kind).unwrap().clone();
                    let task_storage = self.task_storage.clone();

                    self.thread_pool
                        .execute(move || match task_executor.run(task) {
                            Ok(result) => {
                                if let Err(e) = task_storage.push(result) {
                                    println!("Failed to push task result: {}", e);
                                }
                            }
                            Err(e) => {
                                println!("Failed to run task: {}", e);
                            }
                        });
                }
                None => {
                    // No more tasks to run, sleep for a while before checking again
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}
