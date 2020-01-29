use crate::settings::SettingsRef;
use crate::utils::{base64, body, headers, json};
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
    let (client_id, _) = match headers::public_key(&req) {
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
    let json_req: GreetingRequest = match body::read(req)
        .await
        .and_then(|req| base64::read(&req))
        .and_then(|req| shared_secret.read(&req))
        .and_then(|req| json::read(&req))
    {
        // let json_req: GreetingRequest = match body::read(req).await.and_then(|req| json::read(&req)) {
        Ok(json_req) => json_req,
        Err(err) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(err.to_string().into())?)
        }
    };
    // let json_res = json_req;
    // json::res(json::write(&json_res)?)
    let json_res = GreetingResponse {
        hello: json_req.name,
    };
    Ok(Response::new(
        base64::write(&shared_secret.write(&json::write(&json_res)?)?).into(),
    ))
}
