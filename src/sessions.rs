use anyhow::Result;
use ring::aead::UnboundKey;
use std::collections::HashMap;
use std::ops;
use std::sync::{Arc, Mutex};

#[derive(Default, Debug)]
pub struct Sessions(HashMap<String, UnboundKey>);

#[derive(Clone, Default, Debug)]
pub struct SessionsRef(Arc<Mutex<Sessions>>);

impl Sessions {
    pub fn new() -> SessionsRef {
        SessionsRef::default()
    }
}

impl SessionsRef {
    pub fn set(&mut self, key: String, value: UnboundKey) -> Result<()> {
        let mut sessions = self
            .0
            .lock()
            .map_err(|_| anyhow!("Cannot get lock on Sessions"))?;
        sessions.insert(key, value);
        Ok(())
    }
}

impl ops::Deref for Sessions {
    type Target = HashMap<String, UnboundKey>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Sessions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
