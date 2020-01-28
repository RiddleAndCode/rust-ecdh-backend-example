#[macro_use]
extern crate log;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate serde;

mod crypto;
mod keystore;
mod routes;
mod settings;
mod utils;

pub use crate::settings::{Settings, SettingsRef};
pub use crate::utils::algs::*;

use anyhow::Result;
use hyper::{Body, Request, Response};

pub async fn handle(req: Request<Body>, settings: SettingsRef) -> Result<Response<Body>> {
    utils::middleware::log(routes::route, req, settings).await
}
