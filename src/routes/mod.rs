mod greeting;
mod info;
mod session;

use crate::settings::SettingsRef;
use anyhow::Result;
use hyper::{Body, Method, Request, Response, StatusCode};

static NOTFOUND: &[u8] = b"Not Found";

pub async fn route(req: Request<Body>, settings: SettingsRef) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/info") => info::info(req, settings).await,
        (&Method::POST, "/session") => session::create_session(req, settings).await,
        (&Method::POST, "/greeting") => greeting::greeting(req, settings).await,
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap())
        }
    }
}
