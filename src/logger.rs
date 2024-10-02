use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;
use tokio::sync::Mutex;
use std::sync::Arc;

pub struct Logger {
    file: Arc<Mutex<std::fs::File>>,
}

impl Logger {
    pub fn new(log_file: &str) -> Logger {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)
            .expect("Failed to open log file");
        Logger {
            file: Arc::new(Mutex::new(file)),
        }
    }

    pub async fn log(&self, message: &str) {
        let mut file = self.file.lock().await;
        writeln!(file, "{}", message).expect("Failed to write to log file");
        println!("{}", message); // Print to console as well
    }
}
