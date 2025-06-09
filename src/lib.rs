use chrono::Local;
use once_cell::sync::OnceCell;
use std::ffi::{c_char, CStr};
use std::fs::{create_dir, OpenOptions};
use std::io::Write;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    fn new(level: &str) -> LogLevel {
        match level.to_lowercase().as_str() {
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" => LogLevel::Warn,
            "error" => LogLevel::Error,
            _ => LogLevel::Info,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

struct Logger {
    file_path: String,
    file_mutex: Mutex<()>,
}

impl Logger {
    fn new(dir: &str) -> Logger {
        create_dir(dir).expect("create dir error");
        let t = Local::now().format("log-%Y%m%d.log").to_string();
        let file = format!("{}/{}", dir, t);
        Logger {
            file_path: file,
            file_mutex: Mutex::new(()),
        }
    }

    fn write(&self, level: LogLevel, tag: &str, msg: &str) {
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

static LOGGER: OnceCell<Arc<Logger>> = OnceCell::new();

#[unsafe(no_mangle)]
pub extern "C" fn log_init(dir: *const c_char) {
    if dir.is_null() {
        return;
    }

    let dir_str = unsafe { CStr::from_ptr(dir).to_string_lossy() };
    let logger = Arc::new(Logger::new(&dir_str));
    let _ = LOGGER.set(logger);
}

#[unsafe(no_mangle)]
pub extern "C" fn log_write(level: *const c_char, tag: *const c_char, msg: *const c_char) {
    if level.is_null() || tag.is_null() || msg.is_null() {
        return;
    }

    let logger = LOGGER.get();
    if logger.is_none() {
        return; // Not initialized
    }

    let level_str = unsafe { CStr::from_ptr(level).to_string_lossy() };
    let tag_str = unsafe { CStr::from_ptr(tag).to_string_lossy() };
    let msg_str = unsafe { CStr::from_ptr(msg).to_string_lossy() };

    let level_enum = LogLevel::new(&level_str);
    logger.unwrap().write(level_enum, &tag_str, &msg_str);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let logger = Logger::new("./logs");
        for i in 0..50 {
            let tag = "test-tag";
            let msg = format!("This is test log msg, i = {}", i);
            logger.write(LogLevel::Debug, tag, msg.as_str());
            logger.write(LogLevel::Info, tag, msg.as_str());
            logger.write(LogLevel::Warn, tag, msg.as_str());
            logger.write(LogLevel::Error, tag, msg.as_str());
        }
    }
}
