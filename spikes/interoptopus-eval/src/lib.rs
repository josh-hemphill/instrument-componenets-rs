//! Interoptopus 0.16 spike covering the UniFFI hard cases.
//!
//! Mirrors the questions we asked of UniFFI: services as C# classes, async
//! `Task` + `CancellationToken`, C# callbacks, borrowed slices, and typed errors.

use interoptopus::pattern::asynk::Async;
use interoptopus::pattern::result::result_to_ffi;
use interoptopus::rt::Tokio;
use interoptopus::{callback, ffi, AsyncRuntime};

/// Typed FFI error. Service methods returning `Result<_, Error>` throw
/// `EnumException<Error>` in C#.
#[ffi]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Timeout,
    Io,
    Protocol,
}

callback!(ObserverCallback(kind: u32));

/// Sync DMM-shaped service — the C# side should look like a class, not a bag of P/Invokes.
#[ffi(service)]
pub struct Dmm {
    volts: f64,
}

#[ffi]
impl Dmm {
    pub fn create() -> ffi::Result<Self, Error> {
        ffi::Ok(Self { volts: 3.3 })
    }

    pub fn measure_voltage_dc(&self) -> ffi::Result<f64, Error> {
        ffi::Ok(self.volts)
    }

    pub fn fail_timeout(&self) -> ffi::Result<f64, Error> {
        ffi::Err(Error::Timeout)
    }
}

/// Async DMM. Generated C# methods take `CancellationToken` and return `Task`/`Task<T>`.
#[ffi(service)]
#[derive(AsyncRuntime)]
pub struct AsyncDmm {
    runtime: Tokio,
    volts: f64,
}

#[ffi]
impl AsyncDmm {
    pub fn create() -> ffi::Result<Self, Error> {
        result_to_ffi(|| {
            Ok(Self {
                runtime: Tokio::new(),
                volts: 1.25,
            })
        })
    }

    pub async fn measure_voltage_dc(this: Async<Self>) -> ffi::Result<f64, Error> {
        tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        ffi::Ok(this.volts)
    }

    /// Sleeps until cancelled; used to prove `CancellationToken` drops the Rust future.
    pub async fn sleep_forever(_this: Async<Self>) -> ffi::Result<(), Error> {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
        ffi::Ok(())
    }
}

/// Session-shaped service for callbacks and borrowed slices.
#[ffi(service)]
pub struct Session {}

#[ffi]
impl Session {
    pub fn create() -> ffi::Result<Self, Error> {
        ffi::Ok(Self {})
    }

    pub fn ping_observer(&self, callback: ObserverCallback) {
        callback.call(7);
    }

    /// Borrowed view of caller memory (C# `byte[].Slice()`), not an owned copy.
    pub fn checksum_slice(&self, data: ffi::Slice<u8>) -> u32 {
        data.as_slice().iter().map(|&b| b as u32).sum()
    }
}

pub fn inventory() -> interoptopus::inventory::RustInventory {
    use interoptopus::inventory::RustInventory;
    use interoptopus::service;

    RustInventory::new()
        .register(service!(Dmm))
        .register(service!(AsyncDmm))
        .register(service!(Session))
        .validate()
}

#[cfg(test)]
mod tests {
    use super::inventory;
    use interoptopus_csharp::RustLibrary;

    #[test]
    fn generate_bindings() -> Result<(), Box<dyn std::error::Error>> {
        std::fs::create_dir_all("csharp/generated")?;
        RustLibrary::builder(inventory())
            .dll_name("interoptopus_eval")
            .build()
            .process()?
            .write_buffers_to("csharp/generated/")?;
        Ok(())
    }
}
