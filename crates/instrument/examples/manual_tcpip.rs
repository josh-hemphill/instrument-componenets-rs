//! Discover instruments including a manually specified TCPIP/LXI address.
use instrument::prelude::*;

fn main() -> Result<()> {
    let catalog = Discovery::visa()?
        .manual_address("TCPIP0::192.168.0.42::INSTR")
        .scan()?;

    catalog.print_summary();
    Ok(())
}
