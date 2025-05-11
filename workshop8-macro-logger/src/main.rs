trait LoggerTrait {
    fn log(&mut self, message: &str);
    fn flush(&mut self);
}


macro_rules! make_logger {
    ($namespace:expr, $log_level:expr) => {
        {
            struct Logger {
                namespace: String,
                log_level: String,
                buffer: Vec<String>,
            }

            impl LoggerTrait for Logger {
                fn log(&mut self, message: &str) 
                {
                    let formatted_message = format!("[{}] [{}]: {}", self.namespace, self.log_level, message);
                    self.buffer.push(formatted_message);
                }

                fn flush(&mut self) {
                    for message in &self.buffer {
                        println!("{}", message);
                    }
                    self.buffer.clear();
                }
            }

            Logger {
                namespace: $namespace.to_string(),
                log_level: $log_level.to_string(),
                buffer: Vec::new(),
            }
        }
    };
    
}

fn main(){
    let mut instance = make_logger!("namespace", "INFO");
    instance.log("This is a log message");
    instance.flush();
    let mut instance2 = make_logger!("namespace2", "DEBUG");
    instance2.log("This is another log message");
    instance2.flush();
}