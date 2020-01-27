use anyhow::Result;
use bytes::buf::Buf;
use hyper::{Body, Request};

pub async fn decode_body<T>(req: Request<Body>) -> Result<Vec<u8>> {
    Ok(base64::decode(hyper::body::aggregate(req).await?.bytes())?)
}

pub mod module {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::convert::TryFrom;
    use std::fmt;

    pub fn serialize<S, T>(bytes: T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Sized + AsRef<[u8]>,
    {
        serializer.serialize_str(&base64::encode(bytes.as_ref()))
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: TryFrom<Vec<u8>>,
        T::Error: fmt::Display,
    {
        String::deserialize(deserializer)
            .and_then(|string| {
                base64::decode(&string).map_err(|err| serde::de::Error::custom(err.to_string()))
            })
            .and_then(|bytes| {
                T::try_from(bytes).map_err(|err| serde::de::Error::custom(err.to_string()))
            })
    }
}
