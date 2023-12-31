use super::task::TaskRef;
use std::{
    collections::HashMap,
    fmt::{self, Formatter, Debug},
};

/// Represents a queue that holds tasks with unique keys.
pub struct Queue {
    pub queue: HashMap<usize, TaskRef>,
    pub next_key: usize,
}

impl Queue {
    /// Constructs a new `Queue` instance.
    pub fn new() -> Self {
        Queue {
            queue: HashMap::new(),
            next_key: 0,
        }
    }

    /// Adds a task to the queue and returns the key associated with it.
    ///
    /// # Example
    /// ```
    /// let mut queue = Queue::new();
    /// let task = Task::new();
    /// let key = queue.push(task);
    /// ```
    pub fn push(&mut self, task: TaskRef) -> usize {
        let key = self.next_key;
        self.queue.insert(key, task);
        self.next_key += 1;
        key
    }

    // Removes and returns a task by its key, if it exists.
    // pub fn remove(&mut self, key: usize) -> Option<TaskRef> {
    //     self.queue.remove(&key)
    // }

    // Removes and returns the first task in the queue, if any.
    // pub fn pop(&mut self) -> Option<TaskRef> {
    //     let key = self.queue.keys().next().cloned();
    //     key.and_then(|k| self.queue.remove(&k))
    // }

    // Returns the number of tasks in the queue.
    // pub fn len(&self) -> usize {
    //     self.queue.len()
    // }

    // Checks if the queue is empty.
    // pub fn is_empty(&self) -> bool {
    //     self.queue.is_empty()
    // }
}

impl Clone for Queue {
    fn clone(&self) -> Self {
        Queue {
            queue: self.queue.clone(),
            next_key: self.next_key,
        }
    }
}

impl Default for Queue {
    fn default() -> Self {
        Queue::new()
    }
}

impl Debug for Queue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Queue {{ queue: {:?}, next_key: {} }}", self.queue, self.next_key)
    }
}