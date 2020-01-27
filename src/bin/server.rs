#[macro_use]
extern crate log;

use anyhow::{Error, Result};
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::str::FromStr;
use std::{fs, net};

use ecdh_backend::{handle, ConfigBuilder, Sessions};

#[tokio::main]
async fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Info)?;

    let addr = net::SocketAddr::new(net::IpAddr::from_str("0.0.0.0")?, 4000);
    let config = ConfigBuilder::new()
        .key_pair_pkcs8(fs::read("./config/secret.key")?)
        .build()?;
    let sessions = Sessions::new();
    // TODO add config flags

    let make_svc = make_service_fn(move |_conn| {
        let sessions = sessions.clone();
        let config = config.clone();
        async move {
            Ok::<_, Error>(service_fn(move |req| {
                handle(req, sessions.clone(), config.clone())
            }))
        }
    });

    info!("Starting server: {}", addr);
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        error!("server error: {}", e);
    }
    info!("Stopping server...");
    Ok(())
}
