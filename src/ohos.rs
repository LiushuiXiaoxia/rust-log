use chrono::Local;

pub fn init_ohos() {
    println!("{}", "init ohos");
}

pub fn ohos_log(level: &str, tag: &str, msg: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let s = format!("{} [{}] [{}] {}", timestamp, level, tag, msg);
    unsafe {}
    println!("{}", s);
}
