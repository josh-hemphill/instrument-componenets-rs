//! Scan for VISA instruments and print catalog summary.
use instrument::prelude::*;

fn main() -> Result<()> {
    let catalog = Discovery::visa()?.scan()?;
    catalog.print_summary();
    Ok(())
}
