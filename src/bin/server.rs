#[macro_use]
extern crate log;

use anyhow::Error;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use ecdh_backend::{handle, Sessions};

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let addr = SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), 4000);
    // let sessions = Sessions::new();

    let make_svc = make_service_fn(move |_conn| {
        // let sessions = sessions.clone();
        let sessions = Sessions::new();
        async move { Ok::<_, Error>(service_fn(move |req| handle(req, sessions.clone()))) }
    });

    info!("Starting server: {}", addr);
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        error!("server error: {}", e);
    }
    info!("Stopping server...");
}
