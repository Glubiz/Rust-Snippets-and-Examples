use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

type AppState = Arc<Mutex<String>>;

async fn handle_request(
    state: AppState,
    req: Request<Body>,
) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/") => {
            let greeting = state.lock().unwrap();
            Ok(Response::new(Body::from(format!("{}\n", greeting))))
        }
        (&hyper::Method::POST, "/set_greeting") => {
            let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
            let new_greeting = String::from_utf8(body_bytes.to_vec()).unwrap();
            *state.lock().unwrap() = new_greeting;
            Ok(Response::new(Body::from("Greeting updated.\n")))
        }
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = hyper::StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(String::from("Hello, World!")));

    let make_service = make_service_fn(move |_| {
        let state = Arc::clone(&state);
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                handle_request(Arc::clone(&state), req)
            }))
        }
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let listener = TcpListener::bind(&addr).await.unwrap();
    let server = Server::from_tcp(listener).unwrap().serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
