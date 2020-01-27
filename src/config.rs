use anyhow::Result;
use ring::hkdf::{Salt, HKDF_SHA256};
use ring::rand::SystemRandom;
use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};
use std::sync::Arc;

#[derive(Default)]
pub struct ConfigBuilder {
    key_pair_pkcs8: Option<Vec<u8>>,
    hkdf_salt: Vec<u8>,
}

pub struct Config {
    key_pair: EcdsaKeyPair,
    hkdf_salt: Salt,
    rng: SystemRandom,
}

pub type ConfigRef = Arc<Config>;

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn key_pair_pkcs8(mut self, key_pair_pkcs8: Vec<u8>) -> Self {
        self.key_pair_pkcs8 = Some(key_pair_pkcs8);
        self
    }

    pub fn hkdf_salt(mut self, hkdf_salt: Vec<u8>) -> Self {
        self.hkdf_salt = hkdf_salt;
        self
    }

    pub fn build(self) -> Result<ConfigRef> {
        Ok(Arc::new(Config {
            key_pair: self
                .key_pair_pkcs8
                .ok_or_else(|| anyhow!("Keypair missing in config"))
                .and_then(|key_pair_pkcs8| {
                    EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &key_pair_pkcs8)
                        .map_err(|_| anyhow!("Invalid Keypair in config"))
                })?,
            hkdf_salt: Salt::new(HKDF_SHA256, &self.hkdf_salt),
            rng: SystemRandom::new(),
        }))
    }
}

impl Config {
    pub fn key_pair(&self) -> &EcdsaKeyPair {
        &self.key_pair
    }

    pub fn hkdf_salt(&self) -> &Salt {
        &self.hkdf_salt
    }

    pub fn rng(&self) -> &SystemRandom {
        &self.rng
    }
}
