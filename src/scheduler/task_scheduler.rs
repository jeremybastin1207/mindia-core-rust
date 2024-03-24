use std::collections::HashMap;
use std::error::Error;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::time::sleep;
use super::{Task, TaskExecutor, TaskKind, TaskStorage};

pub struct TaskScheduler {
    should_stop: Arc<AtomicBool>,
    task_storage: Arc<dyn TaskStorage>,
    task_executors: HashMap<TaskKind, Arc<dyn TaskExecutor>>,
}

impl TaskScheduler {
    pub fn new(task_storage: Arc<dyn TaskStorage>) -> Self {
        Self {
            should_stop: Arc::new(AtomicBool::new(false)),
            task_storage,
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

                    tokio::spawn(async move {
                        match task_executor.run(task).await {
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
                    sleep(std::time::Duration::from_secs(1)).await;
                }
            }

            sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}

impl Drop for TaskScheduler {
    fn drop(&mut self) {
        self.stop();
    }
}

pub fn run_scheduler(
    task_storage: Arc<dyn TaskStorage>,
    task_executors: HashMap<TaskKind, Arc<dyn TaskExecutor>>,
) -> Arc<TaskScheduler> {
    let mut task_scheduler = TaskScheduler::new(Arc::clone(&task_storage));

    for (task_kind, task_executor) in task_executors {
        task_scheduler.register_task_executor(task_kind, task_executor);
    }

    let task_scheduler = Arc::new(task_scheduler);
    let task_scheduler_clone = task_scheduler.clone();

    tokio::spawn(async move {
        task_scheduler_clone.run().await;
    });

    task_scheduler
}
