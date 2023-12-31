use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}, fmt::{self, Debug, Formatter, format}, error::Error};
use async_trait::async_trait;
use tokio::sync::Mutex;

use super::LockedState;

// Type for passing tasks around
pub type TaskRef = Arc<Mutex<Task>>; 

/* Nicholas Lunna
   Rust - Task - Tokio

   Tasks represent a unit of work that needs to be done.
   Tasks carry behaviors, defined by `TaskBehavior`, that are executed by the worker threads.
   Tasks will carry a TaskState struct that holds all information regarding the Task, its execution, and its status.

   Tasks are fed to the TaskScheduler, which will then give it to a worker to be executed.

    Here is a diagram of the Task struct and its components:
    Why did I make this at 4 AM? I don't know. I think I'm going insane, or something.
    I really like playing guitar. 
    I got this Anniversary Edition Fender Strat, with a nice maple neck and mint blue finish.
    Vyvanse goes brrrrrrrrrrrrr - Nicholas Lunna, December 30, 2023. Happy New Years Eve Eve. 

                                [+++++++++++++] 
                                [-(pub)-Task--]
    [+++++++++++++++++++]       [=============]
    [-(pub)-TaskState---]<-----+[-task_state--] <-M-M-M-M-M-M-M-M-M-M-M-M-M-M-M-M-M 
    [===================]       [p-behaviors--]+-+   <vec                          \         vec>
    [p-status----Status-]       [+++++++++++++]  |   <vec   [+++++++++++++++]       \        vec>
    [p-start_time---u64-]                        |   <vec   [-TaskBehavior--]        \       vec>
    [p-end_time-----u64-]                        +--><vec   [-=============-]         \      vec>
    [p-result--- String-]                            <vec   [p-execute()----] --> [-TskRef-] vec>
    [p-err------String--]                            <vec   [+++++++++++++++]                vec>
    [+++++++++++++++++++]                            <vec                                    vec>
                                                     <++++++++++vector+of+behaviors+++++++++++++>

                                               Nicholas Lunna
                                             December 30, 2023
                                    Title: Task Structure in Rust - Tokio
*/

pub struct TaskEtc {
    pub name : String,
    pub id : usize,
}
impl Clone for TaskEtc {
    fn clone(&self) -> Self {
        TaskEtc {
            name: self.name.clone(),
            id: self.id.clone(),
        }
    }
}
pub struct TaskState {
    pub status:     Status,   // Status of the task
    pub queue_id:   usize,    // ID of the queue that the task is in
    pub start_time: u64,      // Linux time for task start
    pub end_time:   u64,      // Linux time for task end
    pub result:     String,   // Result of the task
    pub err:        String,   // Error message, if any
    pub etc:        TaskEtc,  // Etcetera
}
impl Clone for TaskState {
    fn clone(&self) -> Self {
        TaskState {
            status:     self.status.clone(),
            queue_id:   self.queue_id.clone(),
            start_time: self.start_time.clone(),
            end_time:   self.end_time.clone(),
            result:     self.result.clone(),
            err:        self.err.clone(),
            etc:        self.etc.clone(),
        }
    }
}
impl Debug for TaskState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "TaskState {{ status: {:?}, start_time: {:?}, end_time: {:?}, result: {:?}, err: {:?} }}", self.status, self.start_time, self.end_time, self.result, self.err)
    }
}


pub type BehaviorsSafe = Arc<Vec<Box<dyn TaskBehavior>>>;
pub type Behaviors = Vec<Box<dyn TaskBehavior>>;

pub struct Task {
    pub behaviors: BehaviorsSafe,   // Task can have multiple behaviors
                                                 // e.g., update state and send a response
                                                 // Behaviors are executed asynchronously

    pub task_state: TaskState,                   // Internal state of the task, mainly for debugging
    pub _state: Option<LockedState>,             // State of the game. Only allocated by the TaskScheduler
}

impl Task {
    pub fn new(behaviors: Behaviors, name: String) -> TaskRef {
        let rand_num = rand::random::<u8>() as usize;
        let task_state = TaskState {    
            status:     Status::Pending,                                       
            queue_id:   0,                            
            start_time: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards").as_secs(), 
            end_time:   SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards").as_secs(), 
            result:     String::from(""), 
            err:        String::from(""),
            etc:        TaskEtc {
                        name: format!("Name:{}",rand_num),
                        id: rand_num,
            },
        };

        // Create a new Task with the given behavior and state
        let behaviors: BehaviorsSafe = Arc::new(behaviors);
        let task = Task {
            behaviors,
            task_state,
            _state: None,
        };

        // Return the task wrapped in an Arc<Mutex<Task>> for thread safety and ease of reference
        Arc::new(Mutex::new(task))
    }
}
impl Clone for Task {
    fn clone(&self) -> Self {
        let behaviors: BehaviorsSafe = self.behaviors.clone();
        let task_state: TaskState = self.task_state.clone();
        Task {
            behaviors,
            task_state,
            _state: None,
        }
    }
}
impl Debug for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Task {{ behaviors: {:?}, task_state: {:?} }}", self.behaviors, self.task_state)
    }
}



pub enum Status {
    Pending,
    Queued,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}
impl Clone for Status {
    fn clone(&self) -> Self {
        match self {
            Status::Pending => Status::Pending,
            Status::Queued => Status::Queued,
            Status::InProgress => Status::InProgress,
            Status::Completed => Status::Completed,
            Status::Failed => Status::Failed,
            Status::Cancelled => Status::Cancelled,
        }
    }
}
impl Debug for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Status::Pending => write!(f, "Pending"),
            Status::Queued => write!(f, "Queued"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Completed => write!(f, "Completed"),
            Status::Failed => write!(f, "Failed"),
            Status::Cancelled => write!(f, "Cancelled"),
        }
    }
}
impl Default for Status {
    fn default() -> Self { Status::Pending }
}

#[async_trait]
pub trait TaskBehavior : Send + Sync {
    async fn on_event(&self, task: &Task) -> Result<TaskState, Box<dyn Error + Send + Sync>>;
}

impl Debug for dyn TaskBehavior {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "TaskBehavior {{ }}")
    }
}