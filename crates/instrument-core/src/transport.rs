use crate::address::InterfaceKind;
use crate::connect::ConnectOptions;
use crate::error::{Error, Result, TransportError};
use std::time::Duration;

/// Optional pre-SCPI identity hints from the transport backend.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TransportIdentity {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial: Option<String>,
    pub interface: InterfaceKind,
    pub manf_id: Option<u32>,
    pub model_code: Option<u32>,
}

/// Swappable byte-level transport for instrument I/O.
pub trait Transport: Send {
    fn write(&mut self, data: &[u8]) -> Result<()>;
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    fn clear(&mut self) -> Result<()>;
    fn set_read_timeout(&mut self, timeout: Duration) -> Result<()>;

    /// Reopens a dropped connection (e.g. TCPIP). Default: unsupported.
    fn reconnect(&mut self) -> Result<()> {
        Err(Error::Unsupported("reconnect"))
    }

    /// Pre-SCPI identity hints. Default: empty.
    fn identity(&self) -> TransportIdentity {
        TransportIdentity::default()
    }

    /// Applies connect options (timeouts, etc.).
    fn configure(&mut self, opts: &ConnectOptions) -> Result<()> {
        self.set_read_timeout(opts.read_timeout)
    }
}

/// Boxed transport for dynamic dispatch at the session boundary.
pub type DynTransport = Box<dyn Transport>;

/// In-memory buffer transport for testing read/write plumbing.
#[derive(Debug, Default)]
pub struct BufferTransport {
    pub written: Vec<u8>,
    pub read_data: Vec<u8>,
    read_pos: usize,
}

impl Transport for BufferTransport {
    fn write(&mut self, data: &[u8]) -> Result<()> {
        self.written.extend_from_slice(data);
        Ok(())
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.read_pos >= self.read_data.len() {
            return Err(Error::Transport(TransportError::Closed));
        }
        let n = buf.len().min(self.read_data.len() - self.read_pos);
        buf[..n].copy_from_slice(&self.read_data[self.read_pos..self.read_pos + n]);
        self.read_pos += n;
        Ok(n)
    }

    fn clear(&mut self) -> Result<()> {
        self.read_pos = 0;
        Ok(())
    }

    fn set_read_timeout(&mut self, _timeout: Duration) -> Result<()> {
        Ok(())
    }
}
