use crate::settings::SettingsRef;
use anyhow::Result;
use core::future::Future;
use hyper::{Body, Request, Response};
use std::time::Instant;

pub async fn log<H, R>(
    handler: H,
    req: Request<Body>,
    settings: SettingsRef,
) -> Result<Response<Body>>
where
    H: Fn(Request<Body>, SettingsRef) -> R,
    R: Future<Output = Result<Response<Body>>>,
{
    let time = Instant::now();
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let res = handler(req, settings).await;
    let res_string = match &res {
        Ok(ref res) => format!("{:?}", res.status()),
        Err(ref err) => format!("{}", err),
    };
    info!("{} {} |{:?}| {}", method, path, time.elapsed(), res_string);
    res
}
