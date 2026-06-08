use crate::error::map_visa_error;
use crate::transport::visa_timeout_ms;
use instrument_core::async_transport::AsyncTransport;
use instrument_core::connect::ConnectOptions;
use instrument_core::error::{Error, Result, TransportError};
use instrument_core::transport::TransportIdentity;
use std::future::Future;
use std::io;
use std::pin::Pin;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use visa_rs::enums::attribute::{AttrTmoValue, HasAttribute};
use visa_rs::prelude::Instrument;
use visa_rs::InstrumentTokioAdapter;

fn map_io_error(err: io::Error) -> Error {
    if err.kind() == io::ErrorKind::TimedOut {
        Error::Timeout
    } else {
        Error::Transport(TransportError::Io(err.to_string()))
    }
}

/// Async VISA instrument session transport.
pub struct VisaAsyncTransport {
    adapter: Option<InstrumentTokioAdapter>,
    identity: TransportIdentity,
}

impl VisaAsyncTransport {
    pub fn new(adapter: InstrumentTokioAdapter, identity: TransportIdentity) -> Self {
        Self {
            adapter: Some(adapter),
            identity,
        }
    }

    pub fn adapter(&self) -> Option<&InstrumentTokioAdapter> {
        self.adapter.as_ref()
    }

    pub fn adapter_mut(&mut self) -> Option<&mut InstrumentTokioAdapter> {
        self.adapter.as_mut()
    }

    fn with_sync_instrument<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&Instrument) -> visa_rs::Result<()>,
    {
        let adapter = self
            .adapter
            .take()
            .ok_or_else(|| Error::Transport(TransportError::Closed))?;
        let instr: Instrument = adapter.into();
        let result = f(&instr).map_err(map_visa_error);
        let adapter = InstrumentTokioAdapter::try_from(instr).map_err(map_visa_error)?;
        self.adapter = Some(adapter);
        result
    }
}

impl AsyncTransport for VisaAsyncTransport {
    fn write<'a>(
        &'a mut self,
        data: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let adapter = self
                .adapter
                .as_mut()
                .ok_or_else(|| Error::Transport(TransportError::Closed))?;
            adapter.write_all(data).await.map_err(map_io_error)
        })
    }

    fn read<'a>(
        &'a mut self,
        buf: &'a mut [u8],
    ) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + 'a>> {
        Box::pin(async move {
            let adapter = self
                .adapter
                .as_mut()
                .ok_or_else(|| Error::Transport(TransportError::Closed))?;
            match adapter.read(buf).await {
                Ok(0) => Err(Error::Transport(TransportError::Closed)),
                Ok(n) => Ok(n),
                Err(e) => Err(map_io_error(e)),
            }
        })
    }

    fn clear<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move { self.with_sync_instrument(|instr| instr.clear()) })
    }

    fn set_read_timeout<'a>(
        &'a mut self,
        timeout: Duration,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let attr = AttrTmoValue::new_checked(visa_timeout_ms(timeout))
                .ok_or_else(|| Error::Parse("invalid VISA timeout value".into()))?;
            self.with_sync_instrument(move |instr| instr.set_attr(attr))
        })
    }

    fn identity(&self) -> TransportIdentity {
        self.identity.clone()
    }

    fn configure<'a>(
        &'a mut self,
        opts: &'a ConnectOptions,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        let timeout = opts.read_timeout;
        Box::pin(async move { self.set_read_timeout(timeout).await })
    }
}
