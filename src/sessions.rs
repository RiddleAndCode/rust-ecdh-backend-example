use anyhow::Result;
use std::collections::HashMap;
use std::ops;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default, Debug)]
pub struct Sessions(HashMap<String, String>);

#[derive(Clone, Default, Debug)]
pub struct SessionsRef(Arc<Mutex<Sessions>>);

impl Sessions {
    pub fn new() -> SessionsRef {
        SessionsRef::default()
    }
}

impl SessionsRef {
    pub fn set(&mut self, key: String) -> Result<()> {
        let mut sessions = self
            .0
            .lock()
            .map_err(|_| anyhow!("Cannot get lock on Sessions"))?;
        sessions.insert(key, "hello".to_string());
        Ok(())
    }
}

impl ops::Deref for Sessions {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Sessions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
