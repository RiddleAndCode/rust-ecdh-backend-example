use anyhow::Result;
use ring::rand::SystemRandom;
use ring::signature::{EcdsaKeyPair, KeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};
use std::{env, fs, path};

fn main() -> Result<()> {
    // get output file for secret key
    let file = path::PathBuf::from(env::var("ECDH_CONFIG_HOME").unwrap_or(".".to_string()))
        .join("config")
        .join("secret.key");

    // generate secret key
    let doc = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &SystemRandom::new())?;
    // get public key
    let keypair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, doc.as_ref())?;

    // output public key
    println!("🔑 {}", base64::encode(keypair.public_key().as_ref()));

    // save secret key to file
    let filename = file.to_string_lossy().to_string();
    fs::write(file, doc)?;
    println!("💾 {}", filename);

    Ok(())
}
