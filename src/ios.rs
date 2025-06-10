use chrono::Local;
use objc::runtime::{Class, Object};
use objc::{msg_send, sel, sel_impl};
use objc_foundation::INSString;

pub fn init_ios() {}

extern "C" {
    fn NSLog(format: *const objc::runtime::Object, ...);
}

pub fn ios_log(level: &str, tag: &str, msg: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let s = format!("{} [{}] [{}] {}", timestamp, level, tag, msg);
    unsafe {
        let message = objc_foundation::NSString::from_str(s.as_str());
        let cls = Class::get("NSString").unwrap();
        let string: *mut Object = msg_send![cls, stringWithUTF8String: message.as_str()];
        NSLog(string);
    }
}
