use crate::async_transport::VisaAsyncTransport;
use crate::error::map_visa_error;
use crate::rm::SharedRm;
use crate::session_opener::map_access_mode;
use crate::transport::read_identity_from_instrument;
use instrument_core::address::ResourceAddress;
use instrument_core::async_session::AsyncSessionOpener;
use instrument_core::async_transport::DynAsyncTransport;
use instrument_core::connect::ConnectOptions;
use instrument_core::error::{Error, Result};
use std::ffi::CString;
use std::future::Future;
use std::pin::Pin;
use visa_rs::prelude::AsResourceManager;
use visa_rs::InstrumentTokioAdapter;

/// Opens async VISA sessions for instrument-core.
#[derive(Clone)]
pub struct VisaAsyncSessionOpener {
    rm: SharedRm,
}

impl VisaAsyncSessionOpener {
    pub fn new(rm: SharedRm) -> Self {
        Self { rm }
    }

    pub fn shared_rm(&self) -> SharedRm {
        self.rm.clone()
    }
}

impl AsyncSessionOpener for VisaAsyncSessionOpener {
    fn open<'a>(
        &'a self,
        address: &'a ResourceAddress,
        opts: &'a ConnectOptions,
    ) -> Pin<Box<dyn Future<Output = Result<DynAsyncTransport>> + Send + 'a>> {
        Box::pin(async move {
            let res_id = CString::new(address.raw.as_str())
                .map_err(|e| Error::Parse(e.to_string()))?
                .into();
            let access = map_access_mode(opts.access_mode);

            let instr = self
                .rm
                .strong()
                .open(&res_id, access, opts.open_timeout)
                .map_err(|e: visa_rs::Error| {
                    let msg = e.to_string();
                    if msg.contains("session") || msg.contains("limit") {
                        Error::SessionLimit {
                            address: address.raw.clone(),
                        }
                    } else {
                        map_visa_error(e)
                    }
                })?;

            let identity = read_identity_from_instrument(&instr);
            let adapter = InstrumentTokioAdapter::try_from(instr).map_err(map_visa_error)?;
            let transport = VisaAsyncTransport::new(adapter, identity);
            Ok(Box::new(transport) as DynAsyncTransport)
        })
    }
}
