use crate::utils::crypto;
use anyhow::Result;
use redis::AsyncCommands;
use ring::hkdf;

pub struct KeyStore<'a> {
    client: &'a redis::Client,
    hkdf_salt: &'a hkdf::Salt,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct KeyStoreKey(String);

impl<'a> KeyStore<'a> {
    pub fn new(client: &'a redis::Client, hkdf_salt: &'a hkdf::Salt) -> Self {
        Self { client, hkdf_salt }
    }

    pub async fn set(
        &self,
        key: KeyStoreKey,
        value: crypto::SharedSecretMaterial<'_>,
    ) -> Result<()> {
        Ok(self
            .client
            .get_async_connection()
            .await?
            .set(key.as_ref(), value.as_ref())
            .await?)
    }

    pub async fn get(&self, key: KeyStoreKey) -> Result<crypto::SharedSecret> {
        crypto::SharedSecret::new(
            &crypto::SharedSecretMaterial::new(
                self.client
                    .get_async_connection()
                    .await?
                    .get(key.as_ref())
                    .await?,
            ),
            self.hkdf_salt,
        )
    }
}

impl AsRef<str> for KeyStoreKey {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<String> for KeyStoreKey {
    fn from(string: String) -> Self {
        Self(string)
    }
}
