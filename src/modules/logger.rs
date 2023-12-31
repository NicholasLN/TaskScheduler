// Logger.rs
use tokio::sync::mpsc;
use std::sync::Mutex;
use lazy_static::lazy_static;

pub enum LogMessage {
    Info(String),           //-Informational message--green-
    Warning(String),        //-Warning message -------yellow
    Error(String),          //-Error message ---------red---
}

pub struct Logger {
    receiver: Mutex<mpsc::Receiver<LogMessage>>,
}

impl Logger {
    // Creates a new logger instance with a sender-receiver pair
    pub fn new() -> (Self, mpsc::Sender<LogMessage>) { 
        let (sender, receiver) = mpsc::channel(100);
        let logger = Logger {
            receiver: Mutex::new(receiver),
        };
        (logger, sender)
    }

    // Starts the logger to asynchronously process log messages
    pub async fn start(&self) {
        let mut receiver = self.receiver.lock().unwrap();
        while let Some(message) = receiver.recv().await {
            match message {
                LogMessage::Info(msg) => println!("INFO: {}", msg),
                LogMessage::Warning(msg) => println!("WARNING: {}", msg),
                LogMessage::Error(msg) => println!("ERROR: {}", msg),
            }
        }
    }
}

// Global static instance of the Logger
lazy_static! {
    pub static ref LOGGER: Logger = Logger::new().0;
}