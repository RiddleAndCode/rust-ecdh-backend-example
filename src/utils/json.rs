use anyhow::Result;
use bytes::buf::BufExt;
use hyper::{Body, Request, Response};

pub async fn body<T>(req: Request<Body>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    Ok(serde_json::from_reader(
        hyper::body::aggregate(req).await?.reader(),
    )?)
}

pub fn res<T>(res: &T) -> Result<Response<Body>>
where
    T: serde::Serialize,
{
    Ok(Response::builder()
        .header("Content-type", "application/json")
        .body(serde_json::to_string(res)?.into())?)
}
