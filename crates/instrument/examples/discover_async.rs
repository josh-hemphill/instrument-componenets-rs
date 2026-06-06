//! Async VISA discovery — requires NI-VISA / Keysight VISA installed.
use instrument::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let catalog = AsyncDiscovery::visa()?.scan().await?;
    catalog.print_summary();
    Ok(())
}
