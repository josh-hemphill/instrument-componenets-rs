use crate::error::map_visa_error;
use crate::rm::SharedRm;
use crate::transport::{read_identity_from_instrument, VisaTransport};
use instrument_core::address::ResourceAddress;
use instrument_core::connect::{AccessMode as CoreAccessMode, ConnectOptions};
use instrument_core::error::{Error, Result};
use instrument_core::session::SessionOpener;
use instrument_core::transport::DynTransport;
use std::ffi::CString;
use visa_rs::flags::AccessMode;
use visa_rs::prelude::AsResourceManager;

/// Opens VISA sessions for instrument-core.
#[derive(Clone)]
pub struct VisaSessionOpener {
    rm: SharedRm,
}

impl VisaSessionOpener {
    pub fn new(rm: SharedRm) -> Self {
        Self { rm }
    }

    pub fn shared_rm(&self) -> SharedRm {
        self.rm.clone()
    }
}

impl SessionOpener for VisaSessionOpener {
    fn open(&self, address: &ResourceAddress, opts: &ConnectOptions) -> Result<DynTransport> {
        let res_id = CString::new(address.raw.as_str())
            .map_err(|e| instrument_core::Error::Parse(e.to_string()))?
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
        let transport = VisaTransport::new(instr).with_identity(identity);
        Ok(Box::new(transport))
    }
}

pub(crate) fn map_access_mode(mode: CoreAccessMode) -> AccessMode {
    let mut flags = AccessMode::NO_LOCK;
    if mode.exclusive_lock {
        flags |= AccessMode::EXCLUSIVE_LOCK;
    }
    if mode.shared_lock {
        flags |= AccessMode::SHARED_LOCK;
    }
    flags
}
