use std::{convert::Infallible, net::SocketAddr};

use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};

#[tokio::main]
async fn main() {
    // bind address 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // creates a service from the hello_world function
    let svc = make_service_fn(|_conn: &AddrStream| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(echo))
    });

    let server = Server::bind(&addr).serve(svc);

    if let Err(e) = server.await {
        panic!("server err {}", e);
    };
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body())),

        _ => {
            let mut not_found_resp = Response::default();
            *not_found_resp.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found_resp)
        }
    }
}
