mod modules;
mod tasks;
use modules::{State, Task, TaskScheduler};
use tasks::IncrementTickBehavior;
use std::{sync::Arc, time::Duration};
use tokio::sync::RwLock;
use env_logger;

#[tokio::main]
async fn main() {
    //env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let state = Arc::new(RwLock::new(State::new()));
    let task_scheduler = TaskScheduler::new(state.clone()).await;
    loop {
        // Create a new task with IncrementTickBehavior
        //let task = Task::new(vec![Box::new(IncrementTickBehavior)]);
        // Make a tick event every random 1-5 seconds
        
        let wait = rand::random::<u64>() % 5 + 1;
        tokio::time::sleep(Duration::from_secs(wait)).await;

        let amt = rand::random::<u64>() % 9 + 1;
        for _ in 0..amt {
            let task = Task::new(vec![Box::new(IncrementTickBehavior)], String::from("IncrementTick"));
            task_scheduler.add(task).await;
        }
        
        println!("---------------------------");
    }
}