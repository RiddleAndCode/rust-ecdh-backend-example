use crate::utils::algs::*;
use anyhow::Result;
use ring::{aead, agreement, hkdf, rand};
use std::borrow::Cow;

pub struct SharedSecretMaterial<'a>(Cow<'a, [u8]>);

pub struct SharedSecret {
    key: aead::LessSafeKey,
}

impl SharedSecretMaterial<'static> {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(Cow::Owned(bytes))
    }

    pub fn create(
        private_key: agreement::EphemeralPrivateKey,
        peer_public_key: &[u8],
    ) -> Result<Self> {
        agreement::agree_ephemeral(
            private_key,
            &agreement::UnparsedPublicKey::new(ECDH_ALG, peer_public_key),
            anyhow!("Invalid peer Public Key"),
            |key_material| Ok(SharedSecretMaterial::new(key_material.to_owned())),
        )
    }
}

impl SharedSecret {
    pub fn new(material: &SharedSecretMaterial, hkdf_salt: &hkdf::Salt) -> Result<Self> {
        let unbound: aead::UnboundKey = hkdf_salt
            .extract(material.as_ref())
            .expand(&[], SYM_ENC_ALG)?
            .into();
        Ok(Self {
            key: aead::LessSafeKey::new(unbound),
        })
    }

    pub fn encrypt<T>(&self, data: &mut T) -> Result<()>
    where
        T: AsMut<[u8]> + for<'a> Extend<&'a u8>,
    {
        self.key.seal_in_place_append_tag(
            aead::Nonce::assume_unique_for_key([0; 12]),
            aead::Aad::empty(),
            data,
        )?;
        Ok(())
    }
}

pub fn create_ephemeral_key_pair(
    rng: &rand::SystemRandom,
) -> Result<(agreement::EphemeralPrivateKey, agreement::PublicKey)> {
    let private_key = agreement::EphemeralPrivateKey::generate(ECDH_ALG, rng)?;
    let public_key = private_key.compute_public_key()?;
    Ok((private_key, public_key))
}

impl<'a> AsRef<[u8]> for SharedSecretMaterial<'a> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
