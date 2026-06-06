use crate::kind::InstrumentKind;

pub type Result<T> = std::result::Result<T, Error>;

/// Core error type — VISA-free; backend errors are boxed.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("transport: {0}")]
    Transport(#[from] TransportError),
    #[error("backend: {0}")]
    Backend(Box<dyn std::error::Error + Send + Sync>),
    #[error("SCPI command '{command}' failed: {message}")]
    ScpiCommand { command: String, message: String },
    #[error("operation timed out")]
    Timeout,
    #[error("unsupported: {0}")]
    Unsupported(&'static str),
    #[error("device not found: {address}")]
    DeviceNotFound { address: String },
    #[error("kind {kind:?} not supported at {address}; supported: {supported:?}")]
    UnsupportedKind {
        address: String,
        kind: InstrumentKind,
        supported: Vec<InstrumentKind>,
    },
    #[error("parse error: {0}")]
    Parse(String),
    #[error("session limit reached for {address}")]
    SessionLimit { address: String },
    #[error("invalid address: {0}")]
    InvalidAddress(String),
    #[error("mock script exhausted: expected write")]
    MockExhausted,
    #[error("mock script mismatch: expected {expected:?}, got {actual:?}")]
    MockMismatch { expected: String, actual: String },
    #[error("communication failed at {address} after {attempts} attempt(s): {source}")]
    Comm {
        address: String,
        command: Option<String>,
        attempts: u32,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl Error {
    /// Wraps a backend-specific error (e.g. visa_rs::Error) into the core error type.
    pub fn backend(err: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Backend(Box::new(err))
    }

    /// Attaches device address and command context to a communication error.
    pub fn with_comm_context(
        self,
        address: impl Into<String>,
        command: Option<&str>,
        attempts: u32,
    ) -> Self {
        let address = address.into();
        match self {
            Self::Comm { .. } => self,
            other => Self::Comm {
                address,
                command: command.map(str::to_string),
                attempts,
                source: Box::new(other),
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum TransportError {
    #[error("I/O error: {0}")]
    Io(String),
    #[error("connection closed")]
    Closed,
}
