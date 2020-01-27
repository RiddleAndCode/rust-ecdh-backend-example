#[macro_use]
extern crate log;

use anyhow::{Error, Result};
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

use ecdh_backend::{handle, Settings};

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings::default()?;
    simple_logger::init_with_level(*settings.log_level())?;

    let make_svc = make_service_fn(|_conn| {
        let settings = settings.clone();
        async move { Ok::<_, Error>(service_fn(move |req| handle(req, settings.clone()))) }
    });

    info!("Starting server: {}", settings.address());
    let server = Server::bind(settings.address()).serve(make_svc);

    if let Err(e) = server.await {
        error!("server error: {}", e);
    }
    info!("Stopping server...");
    Ok(())
}
