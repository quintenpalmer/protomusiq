use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::Path;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use hyper::{Body, Method, Request, Response};

#[tokio::main]
async fn main() {
    let port: u16 = 5269;
    let _library_path = Path::new("/home/quinten/storage/media/music/bestexisting");

    let addr = SocketAddr::from(([1, 1, 1, 1], port));

    let make_service = make_service_fn(move |_conn| {
        let service = service_fn(move |req| service_handler(req));
        async move { Ok::<_, Infallible>(service) }
    });

    let server = Server::bind(&addr)
        .http1_keepalive(true)
        .serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

pub async fn service_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match handle_request(req) {
        MyResponse::ReplaceMe => Ok(Response::new(Body::from("to be replaced"))),
    }
}

pub enum MyResponse {
    ReplaceMe,
}

pub fn handle_request(req: Request<Body>) -> MyResponse {
    let path_frags = req
        .uri()
        .path()
        .split('/')
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();

    println!(
        "responding to: {} ({}) ({:?})",
        req.uri().path(),
        req.method(),
        path_frags
    );
    match (req.method(), path_frags.as_slice()) {
        // Serve hard-coded images
        (&Method::GET, []) => MyResponse::ReplaceMe,
        _ => MyResponse::ReplaceMe,
    }
}
