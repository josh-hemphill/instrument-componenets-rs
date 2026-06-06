use crate::error::map_visa_error;
use instrument_core::Result;
use std::sync::Arc;
use visa_rs::prelude::DefaultRM;

/// Shared resource manager — keeps the strong RM alive while sessions exist.
#[derive(Clone)]
pub struct SharedRm {
    inner: Arc<DefaultRM>,
}

impl SharedRm {
    pub fn new() -> Result<Self> {
        let strong = DefaultRM::new().map_err(map_visa_error)?;
        Ok(Self {
            inner: Arc::new(strong),
        })
    }

    pub fn strong(&self) -> &DefaultRM {
        &self.inner
    }
}
