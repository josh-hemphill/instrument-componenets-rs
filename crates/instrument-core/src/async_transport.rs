use crate::connect::ConnectOptions;
use crate::error::{Error, Result};
use crate::transport::TransportIdentity;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

/// Boxed async transport for dynamic dispatch at the session boundary.
pub type DynAsyncTransport = Box<dyn AsyncTransport>;

/// Async byte-level transport for instrument I/O.
pub trait AsyncTransport: Send {
    /// Writes bytes asynchronously.
    fn write<'a>(
        &'a mut self,
        data: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>;

    /// Reads bytes asynchronously.
    fn read<'a>(
        &'a mut self,
        buf: &'a mut [u8],
    ) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + 'a>>;

    /// Clears the transport buffer.
    fn clear<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async { Ok(()) })
    }

    /// Sets the read timeout.
    fn set_read_timeout<'a>(
        &'a mut self,
        _timeout: Duration,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async { Ok(()) })
    }

    /// Reopens a dropped connection (e.g. TCPIP). Default: unsupported.
    fn reconnect<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async { Err(Error::Unsupported("reconnect")) })
    }

    /// Pre-SCPI identity hints. Default: empty.
    fn identity(&self) -> TransportIdentity {
        TransportIdentity::default()
    }

    /// Applies connect options (timeouts, etc.).
    fn configure<'a>(
        &'a mut self,
        opts: &'a ConnectOptions,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        let timeout = opts.read_timeout;
        Box::pin(async move { self.set_read_timeout(timeout).await })
    }
}

/// Wraps a sync [`crate::transport::Transport`] as an async transport (for mocks/tests).
pub struct SyncAsAsyncTransport<T: crate::transport::Transport + Send> {
    inner: T,
}

impl<T: crate::transport::Transport + Send> SyncAsAsyncTransport<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: crate::transport::Transport + Send> AsyncTransport for SyncAsAsyncTransport<T> {
    fn write<'a>(
        &'a mut self,
        data: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move { self.inner.write(data) })
    }

    fn read<'a>(
        &'a mut self,
        buf: &'a mut [u8],
    ) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + 'a>> {
        Box::pin(async move { self.inner.read(buf) })
    }

    fn clear<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move { self.inner.clear() })
    }

    fn set_read_timeout<'a>(
        &'a mut self,
        timeout: Duration,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move { self.inner.set_read_timeout(timeout) })
    }

    fn reconnect<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move { self.inner.reconnect() })
    }

    fn identity(&self) -> TransportIdentity {
        self.inner.identity()
    }

    fn configure<'a>(
        &'a mut self,
        opts: &'a ConnectOptions,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move { self.inner.configure(opts) })
    }
}
