use anyhow::Result;
use ring::{aead, agreement, hkdf, rand};

pub static ECDH_ALG: &'static agreement::Algorithm = &agreement::ECDH_P256;
pub static SYM_ENC_ALG: &'static aead::Algorithm = &aead::AES_128_GCM;

pub fn create_ephemeral_key_pair(
    rng: &rand::SystemRandom,
) -> Result<(agreement::EphemeralPrivateKey, agreement::PublicKey)> {
    let private_key = agreement::EphemeralPrivateKey::generate(ECDH_ALG, rng)?;
    let public_key = private_key.compute_public_key()?;
    Ok((private_key, public_key))
}

pub fn create_shared_secret(
    private_key: agreement::EphemeralPrivateKey,
    peer_public_key: &[u8],
    hkdf_salt: &hkdf::Salt,
) -> Result<aead::UnboundKey> {
    agreement::agree_ephemeral(
        private_key,
        &agreement::UnparsedPublicKey::new(ECDH_ALG, peer_public_key),
        anyhow!("Invalid peer Public Key"),
        |key_material| {
            Ok(hkdf_salt
                .extract(&key_material)
                .expand(&[], SYM_ENC_ALG)?
                .into())
        },
    )
}
