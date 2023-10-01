use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

async fn handle(mut _req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("This is a response")))
}


#[tokio::main]
async fn main() {
    // Construct an address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    // Make service to handle each connection
    let make_service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });

    // Then bind and serve
    let server = Server::bind(&addr).serve(make_service);

    // And run forever
    if let Err(e) = server.await {
        eprintln!("server error: {}", e)
    }

}