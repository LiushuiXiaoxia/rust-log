#[cfg(test)]
mod tests {
    use crate::{init, log_message};
    use std::env;

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        println!("Hello, world!");
        let base = env::current_dir()?;
        let dir = base.join("logs");
        println!("dir = {}", dir.to_str().unwrap());

        init(dir.to_str().unwrap());

        for i in 0..50 {
            let tag = "test-tag";
            let msg = format!("This is test log msg, i = {}", i);
            log_message("debug", &tag, &msg);
            log_message("info", &tag, &msg);
            log_message("warn", &tag, &msg);
            log_message("error", &tag, &msg);
        }

        println!("done");
        Ok(())
    }
}
