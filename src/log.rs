use chrono::Local;
use once_cell::sync::OnceCell;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn new(level: &str) -> LogLevel {
        match level.to_lowercase().as_str() {
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" => LogLevel::Warn,
            "error" => LogLevel::Error,
            _ => LogLevel::Info,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

pub(crate) static LOGGER: OnceCell<Arc<Logger>> = OnceCell::new();

#[derive(Debug)]
pub struct Logger {
    file_path: String,
    file_mutex: Mutex<()>,
}

impl Logger {
    pub fn new(dir: &str) -> Logger {
        if !Path::new(dir).exists() {
            fs::create_dir(dir).expect("create log dir error");
        }
        let t = Local::now().format("rust-%Y%m%d.log").to_string();
        let file = format!("{}/{}", dir, t);
        Logger {
            file_path: file,
            file_mutex: Mutex::new(()),
        }
    }

    pub fn write(&self, level: LogLevel, tag: &str, msg: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let line = format!("{} [{}] [{}] {}\n", timestamp, level.as_str(), tag, msg);

        let _lock = self.file_mutex.lock().unwrap();
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
        {
            let _ = file.write_all(line.as_bytes());
        }
    }
}
