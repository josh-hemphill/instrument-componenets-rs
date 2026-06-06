//! NI-VISA / Keysight VISA backend for instrument-core.
//!
//! Enable via the `visa` feature on `instrument-components`.
//! Optional `tokio` feature re-exports `InstrumentTokioAdapter` for async I/O.
//! Optional `cross-compile` feature forwards to `visa-rs/cross-compile`.
#[cfg(feature = "tokio")]
pub mod async_session_opener;
#[cfg(feature = "tokio")]
pub mod async_transport;
pub mod enumerator;
pub mod error;
pub mod rm;
pub mod session_opener;
pub mod transport;

pub use enumerator::VisaEnumerator;
pub use error::map_visa_error;
pub use rm::SharedRm;
// Re-export for consumers wiring custom discovery
pub use session_opener::VisaSessionOpener;
pub use transport::VisaTransport;
pub use visa_rs;

#[cfg(feature = "tokio")]
pub use async_session_opener::VisaAsyncSessionOpener;
#[cfg(feature = "tokio")]
pub use async_transport::VisaAsyncTransport;
#[cfg(feature = "tokio")]
pub use visa_rs::InstrumentTokioAdapter;
