use crate::error::map_visa_error;
use crate::rm::SharedRm;
use crate::transport::read_identity_from_instrument;
use instrument_core::address::{InterfaceKind, ResourceAddress};
use instrument_core::enumerator::{RawResource, ResourceEnumerator};
use instrument_core::Result;
use std::ffi::CString;
use visa_rs::enums::attribute::AttrIntfType;
use visa_rs::prelude::{AccessMode, AsResourceManager, TIMEOUT_IMMEDIATE};

/// VISA-backed resource enumerator.
pub struct VisaEnumerator {
    rm: SharedRm,
}

impl VisaEnumerator {
    pub fn new(rm: SharedRm) -> Self {
        Self { rm }
    }

    fn intf_type_to_kind(intf: AttrIntfType) -> InterfaceKind {
        if intf == AttrIntfType::VI_INTF_GPIB {
            InterfaceKind::Gpib
        } else if intf == AttrIntfType::VI_INTF_VXI || intf == AttrIntfType::VI_INTF_GPIB_VXI {
            InterfaceKind::Vxi
        } else if intf == AttrIntfType::VI_INTF_ASRL {
            InterfaceKind::Serial
        } else if intf == AttrIntfType::VI_INTF_TCPIP {
            InterfaceKind::Tcpip
        } else if intf == AttrIntfType::VI_INTF_USB {
            InterfaceKind::Usb
        } else if intf == AttrIntfType::VI_INTF_PXI {
            InterfaceKind::Pxi
        } else {
            InterfaceKind::Unknown
        }
    }
}

impl ResourceEnumerator for VisaEnumerator {
    fn list(&self, pattern: &str) -> Result<Vec<RawResource>> {
        let expr = CString::new(pattern)
            .map_err(|e| instrument_core::Error::Parse(e.to_string()))?
            .into();
        let list = self
            .rm
            .strong()
            .find_res_list(&expr)
            .map_err(map_visa_error)?;

        let mut resources = Vec::new();
        for res_id in list {
            let res_id = res_id.map_err(map_visa_error)?;
            let raw = res_id.to_string();
            let mut address = ResourceAddress::parse(&raw)?;
            if let Ok((intf, _)) = self.rm.strong().parse_res(&res_id) {
                address.interface = VisaEnumerator::intf_type_to_kind(intf);
            }

            let mut identity_hint = instrument_core::transport::TransportIdentity {
                interface: address.interface,
                ..Default::default()
            };

            if let Ok(instr) =
                self.rm
                    .strong()
                    .open(&res_id, AccessMode::NO_LOCK, TIMEOUT_IMMEDIATE)
            {
                identity_hint = read_identity_from_instrument(&instr);
                identity_hint.interface = address.interface;
            }

            resources.push(RawResource {
                address,
                identity_hint,
            });
        }
        Ok(resources)
    }
}
