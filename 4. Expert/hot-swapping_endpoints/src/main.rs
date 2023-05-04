// router.rs
use std::collections::HashMap;
use hyper::{Body, Request, Response, Error};

pub struct Router {
    routes: HashMap<String, Box<dyn Fn(&Request<Body>) -> Result<Response<Body>, Error> + Send + Sync>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn add_route<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(&Request<Body>) -> Result<Response<Body>, Error> + 'static + Send + Sync,
    {
        self.routes.insert(path.to_string(), Box::new(handler));
    }

    pub fn route(&self, req: &Request<Body>) -> Result<Response<Body>, Error> {
        if let Some(handler) = self.routes.get(req.uri().path()) {
            handler(req)
        } else {
            let mut not_found = Response::new(Body::from("Not Found"));
            *not_found.status_mut() = hyper::StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

// main.rs
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::sync::{Arc, RwLock};
use router::Router;

async fn handle_request(router: Arc<RwLock<Router>>, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let router = router.read().unwrap();
    router.route(&req)
}

#[tokio::main]
async fn main() {
    let router = Arc::new(RwLock::new(Router::new()));
    router.write().unwrap().add_route("/", |_req| Ok(Response::new(Body::from("Hello, World!"))));

    let addr = ([127, 0, 0, 1], 3000).into();
    let make_svc = make_service_fn(move |_conn| {
        let router = Arc::clone(&router);
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| handle_request(Arc::clone(&router), req)))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
