#[macro_use]
extern crate log;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate serde;

mod base64_serde;
mod sessions;

pub use sessions::{Sessions, SessionsRef};

use anyhow::Result;
use bytes::buf::BufExt;
use core::future::Future;
use hyper::{Body, Method, Request, Response, StatusCode};
use ring::signature::{UnparsedPublicKey, ECDSA_P256_SHA256_FIXED};
use std::time::Instant;

static NOTFOUND: &[u8] = b"Not Found";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateSessionRequest {
    #[serde(with = "base64_serde")]
    ephemeral_public_key: Vec<u8>,

    #[serde(with = "base64_serde")]
    signature: Vec<u8>,
}

fn get_public_key(req: &Request<Body>) -> Result<(String, UnparsedPublicKey<Vec<u8>>)> {
    req.headers()
        .get("X-PublicKey")
        .ok_or_else(|| anyhow!("Public Key not found"))
        .and_then(|public_key| {
            let key_bytes = base64::decode(public_key.to_str()?)?;
            let client_id = base64::encode(&key_bytes); // reencode for consistency
            Ok((
                client_id,
                UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, key_bytes),
            ))
        })
}

async fn create_session(req: Request<Body>, mut sessions: SessionsRef) -> Result<Response<Body>> {
    let (client_id, public_key) = match get_public_key(&req) {
        Ok(pub_key_data) => pub_key_data,
        Err(err) => {
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(err.to_string().into())?)
        }
    };
    let json_req: CreateSessionRequest =
        match serde_json::from_reader(hyper::body::aggregate(req).await?.reader()) {
            Ok(json_req) => json_req,
            Err(err) => {
                return Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(err.to_string().into())?)
            }
        };
    sessions.set(client_id)?;
    println!("{:?}", sessions);
    println!("{:?}", json_req);
    Ok(Response::new("Hello, World!".into()))
}

async fn route(req: Request<Body>, sessions: SessionsRef) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/session") => create_session(req, sessions).await,
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap())
        }
    }
}

async fn log_handle<H, R>(
    handler: H,
    req: Request<Body>,
    sessions: SessionsRef,
) -> Result<Response<Body>>
where
    H: Fn(Request<Body>, SessionsRef) -> R,
    R: Future<Output = Result<Response<Body>>>,
{
    let time = Instant::now();
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let res = handler(req, sessions).await;
    let res_string = match &res {
        Ok(ref res) => format!("{:?}", res.status()),
        Err(ref err) => format!("{}", err),
    };
    info!("{} {} |{:?}| {}", method, path, time.elapsed(), res_string);
    res
}

pub async fn handle(req: Request<Body>, sessions: SessionsRef) -> Result<Response<Body>> {
    log_handle(route, req, sessions).await
}
