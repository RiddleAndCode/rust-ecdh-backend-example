use crate::settings::SettingsRef;
use crate::utils::{base64, json};
use anyhow::Result;
use hyper::{Body, Request, Response};
use ring::signature::KeyPair;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Info {
    #[serde(with = "base64::serde")]
    public_key: Vec<u8>,
}

pub async fn info(_: Request<Body>, settings: SettingsRef) -> Result<Response<Body>> {
    json::res(json::write(&Info {
        public_key: settings.key_pair().public_key().as_ref().to_vec(),
    })?)
}
