use objc::runtime::Object;
use objc::{msg_send, sel, sel_impl};

pub fn init_ios() {}

pub fn ios_log(_level: &str, tag: &str, msg: &str) {
    // unsafe {
    //     let format = format!("[{}] {}", tag, msg);
    //     let nsstring = objc_foundation::NSString::from_str(&format);
    //     let _: () = msg_send![class!(NSLog), NSLog:@"%@", nsstring.as_ptr()];
    // }
}
