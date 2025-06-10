use crate::log::LogLevel;
use android_logger::{Config, FilterBuilder};
use jni::JNIEnv;
use jni::JavaVM;
use libc::{c_char, c_int};
use log::{debug, error, info, warn, LevelFilter};
use once_cell::sync::OnceCell;
use std::ffi::CString;

static JVM: OnceCell<JavaVM> = OnceCell::new();

pub fn init_android(env: &JNIEnv) {
    let jvm = env.get_java_vm().unwrap();
    JVM.set(jvm).ok();

    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Debug) // limit log level
            .with_tag("rust-log") // logs will show under mytag tag
            .with_filter(
                // configure messages for specific crate
                FilterBuilder::new()
                    .parse("debug,hello::crate=error")
                    .build(),
            ),
    );
}

#[cfg(target_os = "android")]
#[link(name = "log")]
extern "C" {
    pub fn __android_log_print(prio: c_int, tag: *const c_char, fmt: *const c_char, ...) -> c_int;
}

pub fn android_log(level: &str, tag: &str, msg: &str) {
    let c_tag = CString::new(tag).unwrap();
    let c_msg = CString::new(msg).unwrap();
    let androidLevel: c_int = match LogLevel::new(level) {
        LogLevel::Debug => 3,
        LogLevel::Info => 4,
        LogLevel::Warn => 5,
        LogLevel::Error => 6,
    };
    unsafe {
        __android_log_print(androidLevel, c_tag.as_ptr(), c_msg.as_ptr());
    }

    match LogLevel::new(level) {
        LogLevel::Debug => {
            debug!("{}", msg);
        }
        LogLevel::Info => {
            info!("{}", msg);
        }
        LogLevel::Warn => {
            warn!("{}", msg);
        }
        LogLevel::Error => {
            error!("{}", msg);
        }
    };
}
