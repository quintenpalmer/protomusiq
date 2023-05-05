mod contenttype;
mod ds;
mod model;
mod routes;

use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use std::path::Path;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

use crate::ds::Datastore;
use crate::routes::service_handler;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Must supply <port>, and <library-path>");
    }

    let port = args[1].parse::<u16>().unwrap();
    let lib_path = Path::new(&args[2]);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let ds = Datastore::new(lib_path.to_path_buf());

    let make_service = make_service_fn(move |_conn| {
        let ds = ds.clone();
        let service = service_fn(move |req| service_handler(req, ds.clone()));
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
