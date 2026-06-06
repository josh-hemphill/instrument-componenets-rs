use instrument_core::address::ResourceAddress;
use instrument_core::connect::ConnectOptions;
use instrument_core::error::Result;
use instrument_core::mock::MockTransport;
use instrument_core::session::SessionOpener;
use instrument_core::transport::DynTransport;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Mock session opener for fixture-based catalogs.
#[derive(Clone, Default)]
pub struct MockSessionOpener {
    templates: Arc<Mutex<HashMap<String, MockTransport>>>,
}

impl MockSessionOpener {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&self, address: &str, transport: MockTransport) {
        self.templates
            .lock()
            .unwrap()
            .insert(address.to_string(), transport);
    }
}

impl SessionOpener for MockSessionOpener {
    fn open(&self, address: &ResourceAddress, _opts: &ConnectOptions) -> Result<DynTransport> {
        let templates = self.templates.lock().unwrap();
        let template =
            templates
                .get(&address.raw)
                .ok_or_else(|| instrument_core::Error::DeviceNotFound {
                    address: address.raw.clone(),
                })?;
        Ok(Box::new(template.reopen()))
    }
}

#[cfg(feature = "tokio")]
use instrument_core::async_session::AsyncSessionOpener;
#[cfg(feature = "tokio")]
use instrument_core::async_transport::DynAsyncTransport;
#[cfg(feature = "tokio")]
use std::future::Future;
#[cfg(feature = "tokio")]
use std::pin::Pin;

/// Mock async session opener for fixture-based catalogs.
#[cfg(feature = "tokio")]
#[derive(Clone, Default)]
pub struct MockAsyncSessionOpener {
    templates: Arc<Mutex<HashMap<String, MockTransport>>>,
}

#[cfg(feature = "tokio")]
impl MockAsyncSessionOpener {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&self, address: &str, transport: MockTransport) {
        self.templates
            .lock()
            .unwrap()
            .insert(address.to_string(), transport);
    }
}

#[cfg(feature = "tokio")]
impl AsyncSessionOpener for MockAsyncSessionOpener {
    fn open<'a>(
        &'a self,
        address: &'a ResourceAddress,
        _opts: &'a ConnectOptions,
    ) -> Pin<Box<dyn Future<Output = Result<DynAsyncTransport>> + Send + 'a>> {
        Box::pin(async move {
            let templates = self.templates.lock().unwrap();
            let template = templates.get(&address.raw).ok_or_else(|| {
                instrument_core::Error::DeviceNotFound {
                    address: address.raw.clone(),
                }
            })?;
            Ok(Box::new(template.reopen()) as DynAsyncTransport)
        })
    }
}
