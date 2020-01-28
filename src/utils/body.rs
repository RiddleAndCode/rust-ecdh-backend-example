use anyhow::Result;
use hyper::{body::Bytes, Body, Request};

pub async fn read(req: Request<Body>) -> Result<Bytes> {
    Ok(hyper::body::to_bytes(req).await?)
}
