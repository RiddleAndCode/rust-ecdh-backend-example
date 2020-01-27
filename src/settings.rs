use crate::keystore::KeyStore;
use crate::utils::algs::{ECDSA_ALG_SIGNING, HKDF_ALG};
use anyhow::Result;
use redis::Commands;
use ring::hkdf::Salt;
use ring::rand::SystemRandom;
use ring::signature::EcdsaKeyPair;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;

pub struct Settings {
    key_pair: EcdsaKeyPair,
    address: SocketAddr,
    log_level: log::Level,
    redis_client: redis::Client,
    hkdf_salt: Salt,
    rng: SystemRandom,
}

pub type SettingsRef = Arc<Settings>;

impl Settings {
    pub const DEFAULT_FILE: &'static str = "config/settings";
    pub const DEFAULT_ENV: &'static str = "ECDH_BACKEND";

    pub const REDIS: &'static str = "redis";
    pub const HKDF_SALT: &'static str = "hkdf_salt";
    pub const KEY_PAIR: &'static str = "key_pair_key";
    pub const HOST: &'static str = "host";
    pub const PORT: &'static str = "port";
    pub const LOG_LEVEL: &'static str = "log_level";
}

impl Settings {
    pub fn default() -> Result<SettingsRef> {
        Self::load(Self::DEFAULT_FILE, Self::DEFAULT_ENV)
    }

    pub fn load(filename: &str, environment_prefix: &str) -> Result<SettingsRef> {
        let mut settings = config::Config::default();
        settings
            .merge(config::File::with_name(filename))?
            .merge(config::Environment::with_prefix(environment_prefix))?;

        let redis_client = redis::Client::open(settings.get_str(Self::REDIS)?)?;
        let key_pair = EcdsaKeyPair::from_pkcs8(
            &ECDSA_ALG_SIGNING,
            redis_client
                .get_connection()?
                .get::<_, Vec<u8>>(settings.get_str(Self::KEY_PAIR)?)?
                .as_ref(),
        )
        .map_err(|_| anyhow!("Invalid Keypair in database"))?;

        Ok(Arc::new(Self {
            address: SocketAddr::new(
                IpAddr::from_str(&settings.get_str(Self::HOST)?)?,
                settings.get_int(Self::PORT)? as u16,
            ),
            log_level: log::Level::from_str(&settings.get_str(Self::LOG_LEVEL)?)?,
            key_pair,
            redis_client,
            hkdf_salt: Salt::new(HKDF_ALG, settings.get_str(Self::HKDF_SALT)?.as_ref()),
            rng: SystemRandom::new(),
        }))
    }

    pub fn log_level(&self) -> &log::Level {
        &self.log_level
    }

    pub fn address(&self) -> &SocketAddr {
        &self.address
    }

    pub fn key_pair(&self) -> &EcdsaKeyPair {
        &self.key_pair
    }

    pub fn redis_client(&self) -> &redis::Client {
        &self.redis_client
    }

    pub fn key_store(&self) -> KeyStore<'_> {
        KeyStore::new(&self.redis_client, &self.hkdf_salt)
    }

    pub fn hkdf_salt(&self) -> &Salt {
        &self.hkdf_salt
    }

    pub fn rng(&self) -> &SystemRandom {
        &self.rng
    }
}
