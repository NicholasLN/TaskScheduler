use tokio::sync::mpsc;
use crate::modules::{TaskRef, LockedState, Worker, Hammer};

/// The `TaskScheduler` struct is responsible for scheduling tasks.
pub struct TaskScheduler {
    tx: mpsc::Sender<TaskRef>,
    state: LockedState,
}

impl TaskScheduler {
    /// Constructs a new `TaskScheduler` instance.
    pub async fn new(state: LockedState) -> Self {
        let (tx, rx) = mpsc::channel::<TaskRef>(100);
        let scheduler = TaskScheduler { tx, state };

        // Instantiate and initiate the worker
        let worker = Worker::new(scheduler.state.clone());
        scheduler.start_worker(worker, rx).await;

        scheduler
    }

    /// Adds a task to the scheduler.
    pub async fn add(&self, task: TaskRef) {
        self.tx.send(task).await.expect("Failed to send task to scheduler");
    }

    /// Initiates the worker to start processing tasks.
    async fn start_worker(&self, mut worker: Worker, mut rx: mpsc::Receiver<TaskRef>) {
        tokio::spawn(async move {
            while let Some(task_ref) = rx.recv().await {
                TaskScheduler::process_task(&mut worker, task_ref).await;
            }
        });
    }

    /// Processes tasks using the provided worker.
    async fn process_task(worker: &mut Worker, task_ref: TaskRef) {
        if let Err(e) = worker.execute(task_ref.clone()).await {
            eprintln!("Error executing task: {}", e);
        }
        task_ref.lock().await._state = None;
    }
}