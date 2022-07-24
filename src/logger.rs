pub struct Logger {}

impl Logger {
    pub fn new() -> Self {
        Logger {}
    }

    pub fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}
