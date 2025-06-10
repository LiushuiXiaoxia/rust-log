use crate::log::{LogLevel, Logger};
use std::ffi::{CStr, c_char, c_void};
use std::sync::Arc;

mod log;
mod test;

#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "ios")]
mod ios;

pub fn init(dir: &str) {
    log::LOGGER
        .set(Arc::new(Logger::new(dir)))
        .expect("init logger error");
}

pub fn log_message(level: &str, tag: &str, msg: &str) {
    #[cfg(target_os = "android")]
    android::android_log(level, tag, msg);

    #[cfg(target_os = "ios")]
    ios::ios_log(level, tag, msg);

    log::LOGGER
        .get()
        .unwrap()
        .write(LogLevel::new(level), tag, msg);
}

#[unsafe(no_mangle)]
pub extern "C" fn log_init(_ctx: *mut c_void, dir: *const c_char) {
    if dir.is_null() {
        return;
    }
    #[cfg(target_os = "android")]
    {
        use jni::JNIEnv;

        // 安全转换上下文为 JNIEnv 指针（必须来自 JNI 环境传入）
        let env_ptr = _ctx as *mut jni::sys::JNIEnv;
        let env = unsafe { JNIEnv::from_raw(env_ptr).expect("Invalid JNIEnv") };

        // let jvm = env.get_java_vm().expect("Failed to get JavaVM");
        android::init_android(&env);
    }

    #[cfg(target_os = "ios")]
    {
        // iOS 忽略 _ctx，直接初始化日志系统
        ios::init_ios();
    }

    let dir_str = unsafe { CStr::from_ptr(dir).to_string_lossy() };
    init(&dir_str);
}

#[unsafe(no_mangle)]
pub extern "C" fn log_write(level: *const c_char, tag: *const c_char, msg: *const c_char) {
    if level.is_null() || tag.is_null() || msg.is_null() {
        return;
    }

    let level_str = unsafe { CStr::from_ptr(level).to_string_lossy() };
    let tag_str = unsafe { CStr::from_ptr(tag).to_string_lossy() };
    let msg_str = unsafe { CStr::from_ptr(msg).to_string_lossy() };

    log_message(&level_str, &tag_str, &msg_str);
}
