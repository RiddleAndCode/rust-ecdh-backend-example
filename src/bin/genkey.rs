use anyhow::Result;
use ecdh_backend::{Settings, ECDSA_ALG_SIGNING};
use redis::Commands;
use ring::rand::SystemRandom;
use ring::signature::{EcdsaKeyPair, KeyPair};

fn main() -> Result<()> {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name(Settings::DEFAULT_FILE))?
        .merge(config::Environment::with_prefix(Settings::DEFAULT_ENV))?;
    let redis_client = redis::Client::open(settings.get_str(Settings::REDIS)?)?;

    // generate secret key
    let doc = EcdsaKeyPair::generate_pkcs8(ECDSA_ALG_SIGNING, &SystemRandom::new())?;
    // get public key
    let keypair = EcdsaKeyPair::from_pkcs8(ECDSA_ALG_SIGNING, doc.as_ref())?;

    // output public key
    println!("🔑 {}", base64::encode(keypair.public_key().as_ref()));

    // save secret key to redis
    let dest_key = settings.get_str(Settings::KEY_PAIR)?;
    redis_client
        .get_connection()?
        .set(&dest_key, doc.as_ref())?;
    println!("💾 Saved in key storage: '{}'", dest_key);

    Ok(())
}
