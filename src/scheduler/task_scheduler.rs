use std::collections::HashMap;
use std::error::Error;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use super::{Task, TaskExecutor, TaskKind, TaskStorage, ThreadPool};

pub struct TaskScheduler {
    should_stop: Arc<AtomicBool>,
    task_storage: Arc<dyn TaskStorage>,
    thread_pool: ThreadPool,
    task_executors: HashMap<TaskKind, Arc<dyn TaskExecutor>>,
}

impl TaskScheduler {
    pub fn new(task_storage: Arc<dyn TaskStorage>) -> Self {
        Self {
            should_stop: Arc::new(AtomicBool::new(false)),
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

    pub fn stop(&self) {
        println!("Shutting down scheduler");
        self.should_stop.store(true, Ordering::Relaxed);
    }

    pub async fn run(&self) {
        while !self.should_stop.load(Ordering::Relaxed) {
            match self.task_storage.pop_queued().unwrap() {
                Some(task) => {
                    let task_executor = self.task_executors.get(&task.kind).unwrap().clone();
                    let task_storage = self.task_storage.clone();

                    actix_web::rt::spawn(async move {
                        match task_executor.run(task) {
                            Ok(result) => {
                                if let Err(e) = task_storage.push(result) {
                                    println!("Failed to push task result: {}", e);
                                }
                            }
                            Err(e) => {
                                println!("Failed to run task: {}", e);
                            }
                        }
                    });
                }
                None => {
                    // No more tasks to run, sleep for a while before checking again
                    actix_web::rt::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }

            actix_web::rt::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}
