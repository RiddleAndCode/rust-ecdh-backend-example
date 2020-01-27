#[macro_use]
extern crate log;

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate serde;

mod base64_serde;
mod config;
mod crypto;
mod sessions;

pub use config::{ConfigBuilder, ConfigRef};
pub use sessions::{Sessions, SessionsRef};

use anyhow::Result;
use bytes::buf::BufExt;
use core::future::Future;
use hyper::{Body, Method, Request, Response, StatusCode};
use ring::signature::{UnparsedPublicKey, ECDSA_P256_SHA256_FIXED};
use std::time::Instant;

static NOTFOUND: &[u8] = b"Not Found";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PublicKeyExchangeMessage {
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

async fn get_json_body<T>(req: Request<Body>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    Ok(serde_json::from_reader(
        hyper::body::aggregate(req).await?.reader(),
    )?)
}

fn build_json_res<T>(res: &T) -> Result<Response<Body>>
where
    T: serde::Serialize,
{
    Ok(Response::builder()
        .header("Content-type", "application/json")
        .body(serde_json::to_string(res)?.into())?)
}

async fn create_session(
    req: Request<Body>,
    mut sessions: SessionsRef,
    config: ConfigRef,
) -> Result<Response<Body>> {
    let (client_id, public_key) = match get_public_key(&req) {
        Ok(pub_key_data) => pub_key_data,
        Err(err) => {
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(err.to_string().into())?)
        }
    };
    let json_req: PublicKeyExchangeMessage = match get_json_body(req).await {
        Ok(json_req) => json_req,
        Err(err) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(err.to_string().into())?)
        }
    };
    match public_key.verify(
        json_req.ephemeral_public_key.as_ref(),
        json_req.signature.as_ref(),
    ) {
        Ok(()) => (),
        Err(_) => {
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body("Bad Signature".into())?)
        }
    };
    let (ephemeral_private_key, ephemeral_public_key) =
        crypto::create_ephemeral_key_pair(config.rng())?;
    let shared_secret = match crypto::create_shared_secret(
        ephemeral_private_key,
        &json_req.ephemeral_public_key,
        config.hkdf_salt(),
    ) {
        Ok(shared_secret) => shared_secret,
        Err(err) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(err.to_string().into())?)
        }
    };
    sessions.set(client_id, shared_secret)?;
    let signature = config
        .key_pair()
        .sign(config.rng(), ephemeral_public_key.as_ref())?
        .as_ref()
        .to_owned();
    let json_res = PublicKeyExchangeMessage {
        ephemeral_public_key: ephemeral_public_key.as_ref().to_owned(),
        signature,
    };
    build_json_res(&json_res)
}

async fn route(
    req: Request<Body>,
    sessions: SessionsRef,
    config: ConfigRef,
) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/session") => create_session(req, sessions, config).await,
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
    config: ConfigRef,
) -> Result<Response<Body>>
where
    H: Fn(Request<Body>, SessionsRef, ConfigRef) -> R,
    R: Future<Output = Result<Response<Body>>>,
{
    let time = Instant::now();
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let res = handler(req, sessions, config).await;
    let res_string = match &res {
        Ok(ref res) => format!("{:?}", res.status()),
        Err(ref err) => format!("{}", err),
    };
    info!("{} {} |{:?}| {}", method, path, time.elapsed(), res_string);
    res
}

pub async fn handle(
    req: Request<Body>,
    sessions: SessionsRef,
    config: ConfigRef,
) -> Result<Response<Body>> {
    log_handle(route, req, sessions, config).await
}
