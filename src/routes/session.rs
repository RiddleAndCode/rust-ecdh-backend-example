use crate::crypto;
use crate::settings::SettingsRef;
use crate::utils::{base64, body, headers, json};
use anyhow::Result;
use hyper::{Body, Request, Response, StatusCode};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PublicKeyExchangeMessage {
    #[serde(with = "base64::serde")]
    ephemeral_public_key: Vec<u8>,

    #[serde(with = "base64::serde")]
    signature: Vec<u8>,
}

pub async fn create_session(req: Request<Body>, settings: SettingsRef) -> Result<Response<Body>> {
    let (client_id, public_key) = match headers::public_key(&req) {
        Ok(pub_key_data) => pub_key_data,
        Err(err) => {
            return Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(err.to_string().into())?)
        }
    };
    let json_req: PublicKeyExchangeMessage =
        match body::read(req).await.and_then(|req| json::read(&req)) {
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
        crypto::create_ephemeral_key_pair(settings.rng())?;
    let shared_secret = match crypto::SharedSecretMaterial::create(
        ephemeral_private_key,
        &json_req.ephemeral_public_key,
    ) {
        Ok(shared_secret) => shared_secret,
        Err(err) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(err.to_string().into())?)
        }
    };
    settings.key_store().set(client_id, shared_secret).await?;
    let signature = settings
        .key_pair()
        .sign(settings.rng(), ephemeral_public_key.as_ref())?
        .as_ref()
        .to_owned();
    let json_res = PublicKeyExchangeMessage {
        ephemeral_public_key: ephemeral_public_key.as_ref().to_owned(),
        signature,
    };
    json::res(json::write(&json_res)?)
}
