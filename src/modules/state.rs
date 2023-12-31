use std::sync::Arc;
use tokio::sync::RwLock;

/// Struct `State` to hold the state of the application.    
/// Accessible in every task through `Task._state`.
/// `State` is wrapped in a `RwLock` to allow for multiple tasks to read from it at the same time.
pub type LockedState = Arc<RwLock<State>>; 

pub struct TimeData {
    pub tick_count: u64
}
pub struct State {
    pub _time: TimeData
}
impl State {
    pub fn new() -> State {
        State {
            _time: TimeData {
                tick_count: 0
            }
        }
    }
}