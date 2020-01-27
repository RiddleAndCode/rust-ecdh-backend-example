use crate::settings::SettingsRef;
use crate::utils::{headers, json};
use anyhow::Result;
use hyper::{Body, Request, Response, StatusCode};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GreetingRequest {
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GreetingResponse {
    hello: String,
}

pub async fn greeting(req: Request<Body>, settings: SettingsRef) -> Result<Response<Body>> {
    let (client_id, public_key) = match headers::public_key(&req) {
        Ok(pub_key_data) => pub_key_data,
        Err(err) => {
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(err.to_string().into())?)
        }
    };
    let shared_secret = match settings.key_store().get(client_id).await {
        Ok(shared_secret) => shared_secret,
        Err(err) => {
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(err.to_string().into())?)
        }
    };
    let json_req: GreetingRequest = match json::body(req).await {
        Ok(json_req) => json_req,
        Err(err) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(err.to_string().into())?)
        }
    };
    // let json_res = GreetingResponse {
    //     hello: json_req.name,
    // };
    let json_res = json_req;
    let mut bytes = serde_json::to_string(&json_res)?.into_bytes();
    shared_secret.encrypt(&mut bytes)?;
    Ok(Response::new(base64::encode(&bytes).into()))
}
