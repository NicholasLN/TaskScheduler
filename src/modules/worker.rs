use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;
use std::error::Error;

use super::{
    TaskRef, 
    TaskState, 
    Queue, 
    Status, 
    LockedState
};

/// Trait `Hammer` to be implemented by `Worker`.
#[async_trait]
pub trait Hammer: Send + Sync {
    async fn execute(&mut self, task: TaskRef) -> Result<TaskState, Box<dyn Error + Send>>;
}

/// Struct `Worker` responsible for task execution.
pub struct Worker {
    tasks: Arc<Mutex<Queue>>,
    _state: LockedState,
}

impl Worker {
    /// Constructs a new `Worker`.
    pub fn new(_state: LockedState) -> Self {
        Worker {
            tasks: Arc::new(Mutex::new(Queue::new())),
            _state,
        }
    }

    /// Updates the state of a given task.
    async fn update_task_state(&self, task: &TaskRef) {
        let mut locked_task = task.lock().await;
        locked_task.task_state.status = Status::Queued;
        
        {
            let tasks = self.tasks.lock().await;
            locked_task.task_state.queue_id = tasks.next_key;
        }
    }

    /// Executes the behaviors associated with a task.
    async fn execute_behaviors(task: &TaskRef, state: &LockedState) {
        let mut locked_task = task.lock().await;
        locked_task._state = Some(state.clone());

        for behavior in locked_task.behaviors.iter() {
            if let Err(e) = behavior.on_event(&*locked_task).await {
                eprintln!("Error executing behavior: {}", e);
            }
        }

        locked_task._state = None;
    }
}


///
/// Why 'Hammer'?
/// Because... "Why not Zoidberg?"
#[async_trait]
impl Hammer for Worker {
    /// Executes a task and returns its state.
    async fn execute(&mut self, task: TaskRef) -> Result<TaskState, Box<dyn Error + Send>> {
        self.update_task_state(&task).await;

        {
            let mut tasks = self.tasks.lock().await;
            tasks.push(task.clone());
        }

        Self::execute_behaviors(&task, &self._state).await;
        let mut locked_task = task.lock().await;

        // Now set it to complete and set its queue_id to None and decrement the queue count
        let mut queue = self.tasks.lock().await;
        locked_task.task_state.status = Status::Completed;
        //locked_task.task_state.queue_id = 0;
        queue.next_key -= 1;

        Ok(locked_task.task_state.clone())
    }
}