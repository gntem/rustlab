use std::collections::HashMap;

struct Invoker {
    ctx: HashMap<String, String>,
    handlers: HashMap<String, Box<dyn Fn(&mut HashMap<String, String>) + 'static>>,
}

impl Invoker {
    fn new(ctx: HashMap<String, String>) -> Self {
        Invoker {
            ctx,
            handlers: HashMap::new(),
        }
    }

    fn register_handler<F>(&mut self, name: &str, handler: F)
    where
        F: Fn(&mut HashMap<String, String>) + 'static,
    {
        self.handlers.insert(name.to_string(), Box::new(handler));
    }

    fn invoke(&mut self, name: &str) {
        if let Some(handler) = self.handlers.get(name) {
            handler(&mut self.ctx);
        } else {
            println!("handler not found");
        }
    }
}

fn main () {
    let mut ctx = HashMap::new();
    ctx.insert("key".to_string(), "a".to_string());

    let mut invoker = Invoker::new(ctx);

    invoker.register_handler("basic_handler", |c| {
        println!("Hello, {}!", c.get("key").unwrap());
        // Example modification:
        c.insert("key".to_string(), "b".to_string());
    });

    invoker.invoke("basic_handler");
    invoker.invoke("basic_handler");
    invoker.invoke("goodbye");
}