use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Handler = Box<dyn Fn() + Send + Sync>;

pub struct Router {
    routes: Arc<Mutex<HashMap<String, Handler>>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_route(&mut self, path: &str, handler: Handler) {
        self.routes.lock().unwrap().insert(path.to_string(), handler);
    }

    pub fn handle_request(&self, path: &str) {
        if let Some(handler) = self.routes.lock().unwrap().get(path) {
            (handler)();
        } else {
            println!("404: Not Found");
        }
    }
}

fn main() {
    let mut router = Router::new();
    router.add_route("/", || {
        println!("Hello, world!");
    });

    router.add_route("/greet", || {
        println!("Greetings!");
    });

    router.handle_request("/");
    router.handle_request("/greet");
}
