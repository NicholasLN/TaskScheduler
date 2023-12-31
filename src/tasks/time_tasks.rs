
use crate::modules::{Task, TaskState, TaskBehavior};
use std::error::Error;
use async_trait::async_trait;

pub struct IncrementTickBehavior;

#[async_trait]
impl TaskBehavior for IncrementTickBehavior {
    async fn on_event(&self, task: &Task) -> Result<TaskState, Box<dyn Error + Send + Sync>> {
        // See if _state is set on the task and not None
        if let Some(state) = &task._state {
            let mut state = state.write().await;
            let task_state = task.task_state.clone();
            state._time.tick_count += 1;
            println!("{}|Tick: {}", task_state.etc.id, state._time.tick_count);
        }
        Ok(task.task_state.clone())
    }
}
