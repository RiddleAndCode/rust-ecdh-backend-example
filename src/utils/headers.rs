use super::algs::ECDSA_ALG;
use crate::keystore::KeyStoreKey;
use anyhow::Result;
use hyper::{Body, Request};
use ring::signature::UnparsedPublicKey;

pub fn public_key(req: &Request<Body>) -> Result<(KeyStoreKey, UnparsedPublicKey<Vec<u8>>)> {
    req.headers()
        .get("X-PublicKey")
        .ok_or_else(|| anyhow!("Public Key not found"))
        .and_then(|public_key| {
            let key_bytes = base64::decode(public_key.to_str()?)?;
            let client_id = KeyStoreKey::from(base64::encode(&key_bytes)); // reencode for consistency
            Ok((client_id, UnparsedPublicKey::new(ECDSA_ALG, key_bytes)))
        })
}
