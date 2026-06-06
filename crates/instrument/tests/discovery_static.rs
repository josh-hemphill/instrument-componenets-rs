use instrument::prelude::*;
use instrument_core::enumerator::{RawResource, StaticEnumerator};
use instrument_core::ModelRegistry;
use std::sync::Arc;

#[test]
fn static_enumerator_merge_and_classify() {
    let addr1 = ResourceAddress::parse("USB0::0x0957::0x0607::SN1::INSTR").unwrap();
    let addr2 = ResourceAddress::parse("GPIB0::10::INSTR").unwrap();

    let enumerator = Arc::new(StaticEnumerator::new(vec![
        RawResource {
            address: addr1,
            identity_hint: TransportIdentity::default(),
        },
        RawResource {
            address: addr2,
            identity_hint: TransportIdentity::default(),
        },
    ]));

    let opener = Arc::new(instrument::mock_backend::MockSessionOpener::new());
    let discovery = Discovery::new(enumerator, opener, ModelRegistry::embedded());
    let catalog = discovery.scan().unwrap();

    assert_eq!(catalog.devices().len(), 2);
    let usb = catalog.device("USB0::0x0957::0x0607::SN1::INSTR").unwrap();
    assert!(usb.supported_kinds().contains(&InstrumentKind::Dmm));
}
