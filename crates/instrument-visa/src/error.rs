use instrument_core::Error;

/// Converts a visa-rs error into the core backend error type.
pub fn map_visa_error(err: visa_rs::Error) -> Error {
    Error::backend(err)
}
