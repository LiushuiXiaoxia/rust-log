use jni::JNIEnv;
use jni::objects::JString;
use jni::JavaVM;
use once_cell::sync::OnceCell;

static JVM: OnceCell<JavaVM> = OnceCell::new();

pub fn init_jvm(env: &JNIEnv) {
    let jvm = env.get_java_vm().unwrap();
    JVM.set(jvm).ok();
}

pub fn android_log(level: &str, tag: &str, msg: &str) {
    if let Some(jvm) = JVM.get() {
        let env = jvm.attach_current_thread().unwrap();
        let log_class = env.find_class("android/util/Log").unwrap();
        let tag_j = env.new_string(tag).unwrap();
        let msg_j = env.new_string(msg).unwrap();
        let method = match level {
            "info" => "i",
            "warn" => "w",
            "error" => "e",
            _ => "d",
        };
        let _ = env.call_static_method(
            log_class,
            method,
            "(Ljava/lang/String;Ljava/lang/String;)I",
            &[tag_j.into(), msg_j.into()],
        );
    }
}
