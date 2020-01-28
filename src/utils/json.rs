use anyhow::Result;
use hyper::{body::Bytes, Body, Response};

pub fn read<T>(body: &Bytes) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    Ok(serde_json::from_slice(body.as_ref())?)
}

pub fn write<T>(res: &T) -> Result<Bytes>
where
    T: serde::Serialize,
{
    Ok(serde_json::to_string(res)?.into())
}

pub fn res(body: Bytes) -> Result<Response<Body>> {
    Ok(Response::builder()
        .header("Content-type", "application/json")
        .body(body.into())?)
}
