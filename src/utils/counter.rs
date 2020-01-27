use ring::aead::{Nonce, NonceSequence};
use std::iter::Iterator;
use std::ops;

#[derive(Default, Clone, Copy, Debug)]
pub struct Count(u64);

#[derive(Default, Clone, Copy, Debug)]
pub struct MonotonicCounter(Count);

impl Iterator for MonotonicCounter {
    type Item = Count;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.0;
        match item.0 {
            std::u64::MAX => None,
            num => {
                self.0 = Count(num + 1);
                Some(item)
            }
        }
    }
}

impl NonceSequence for MonotonicCounter {
    fn advance(&mut self) -> Result<Nonce, ring::error::Unspecified> {
        match self.next() {
            Some(item) => Ok(item.into()),
            _ => Err(ring::error::Unspecified),
        }
    }
}

impl From<Count> for Nonce {
    fn from(count: Count) -> Self {
        let mut bytes = [0u8; 12];
        bytes.copy_from_slice(&count.0.to_le_bytes());
        Nonce::assume_unique_for_key(bytes)
    }
}

impl ops::Deref for Count {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Count {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
