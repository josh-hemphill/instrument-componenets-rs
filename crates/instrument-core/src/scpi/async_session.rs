use super::framing::extract_response;
use super::protocol::{max_write_attempts, normalize_command, parse_f64, SessionCapabilities};
use crate::async_transport::{AsyncTransport, DynAsyncTransport};
use crate::connect::ConnectOptions;
use crate::diagnostics::{CommsEventKind, Diagnostics};
use crate::error::{Error, Result};
use crate::ieee4882::AsyncIeee4882;
use std::time::{Duration, Instant};

/// Async SCPI session over a boxed transport.
pub struct AsyncScpiSession {
    transport: DynAsyncTransport,
    opts: ConnectOptions,
    capabilities: SessionCapabilities,
    read_buffer: Vec<u8>,
    diagnostics: Option<Diagnostics>,
    pending_command: Option<String>,
}

impl AsyncScpiSession {
    pub async fn new(mut transport: DynAsyncTransport, opts: ConnectOptions) -> Result<Self> {
        transport.configure(&opts).await?;
        let mut session = Self {
            transport,
            opts,
            capabilities: SessionCapabilities::default(),
            read_buffer: Vec::with_capacity(4096),
            diagnostics: None,
            pending_command: None,
        };
        if session.opts.reset_on_connect {
            let _ = AsyncIeee4882::new(&mut session).clear_status().await;
            let _ = AsyncIeee4882::new(&mut session).reset().await;
        }
        Ok(session)
    }

    pub fn with_diagnostics(mut self, diagnostics: Diagnostics) -> Self {
        self.diagnostics = Some(diagnostics);
        self
    }

    pub fn transport(&self) -> &dyn AsyncTransport {
        self.transport.as_ref()
    }

    pub fn options(&self) -> &ConnectOptions {
        &self.opts
    }

    /// Drains pending bytes from the transport read buffer.
    pub async fn flush(&mut self) -> Result<()> {
        let short = Duration::from_millis(50);
        self.transport.set_read_timeout(short).await?;
        let mut chunk = [0u8; 256];
        loop {
            match self.transport.read(&mut chunk).await {
                Ok(0) => break,
                Ok(_) => continue,
                Err(Error::Timeout) => break,
                Err(e) => return Err(e),
            }
        }
        self.read_buffer.clear();
        Ok(())
    }

    /// Writes a command without expecting a response.
    pub async fn write(&mut self, command: &str) -> Result<()> {
        self.write_with_retry(command, false).await
    }

    /// Sends a query and returns the response string.
    pub async fn query(&mut self, command: &str) -> Result<String> {
        self.query_with_timeout(command, self.effective_read_timeout())
            .await
    }

    pub async fn query_with_timeout(&mut self, command: &str, timeout: Duration) -> Result<String> {
        self.write_with_retry(command, true).await?;
        let bytes = self.read_response(timeout).await?;
        Ok(String::from_utf8_lossy(&bytes).trim().to_string())
    }

    async fn write_with_retry(&mut self, command: &str, idempotent: bool) -> Result<()> {
        let payload = normalize_command(command, &self.opts.terminator);
        let data = payload.as_bytes();

        let mut attempts = 0;
        let max_attempts = max_write_attempts(idempotent, self.opts.retries);

        loop {
            attempts += 1;
            let started = Instant::now();
            self.pending_command = Some(command.to_string());
            match self.transport.write(data).await {
                Ok(()) => {
                    self.record_success(CommsEventKind::WriteOk, Some(command), attempts, started);
                    return Ok(());
                }
                Err(Error::Timeout) if attempts < max_attempts => {
                    self.record_failure(
                        CommsEventKind::Timeout,
                        Some(command),
                        attempts,
                        started,
                        "write timeout",
                    );
                    if self.opts.reconnect_on_failure {
                        let _ = self.transport.reconnect().await;
                        self.record_reconnect();
                    }
                    tokio::time::sleep(self.opts.retry_backoff).await;
                }
                Err(Error::Timeout) => {
                    self.record_failure(
                        CommsEventKind::Timeout,
                        Some(command),
                        attempts,
                        started,
                        "write timeout",
                    );
                    return Err(Error::Timeout);
                }
                Err(e) => {
                    self.record_failure(
                        CommsEventKind::WriteFailed,
                        Some(command),
                        attempts,
                        started,
                        &e.to_string(),
                    );
                    return Err(e);
                }
            }
        }
    }

    async fn read_response(&mut self, timeout: Duration) -> Result<Vec<u8>> {
        self.transport.set_read_timeout(timeout).await?;
        self.read_buffer.clear();
        let command = self.pending_command.clone();

        let mut chunk = [0u8; 1024];
        loop {
            let started = Instant::now();
            match self.transport.read(&mut chunk).await {
                Ok(0) => tokio::time::sleep(Duration::from_millis(1)).await,
                Ok(n) => {
                    self.read_buffer.extend_from_slice(&chunk[..n]);
                    if let Ok((payload, _)) =
                        extract_response(&self.read_buffer, &self.opts.terminator)
                    {
                        self.record_success(CommsEventKind::ReadOk, command.as_deref(), 1, started);
                        return Ok(payload);
                    }
                }
                Err(Error::Timeout) => {
                    if !self.read_buffer.is_empty() {
                        if let Ok((payload, _)) =
                            extract_response(&self.read_buffer, &self.opts.terminator)
                        {
                            self.record_success(
                                CommsEventKind::ReadOk,
                                command.as_deref(),
                                1,
                                started,
                            );
                            return Ok(payload);
                        }
                    }
                    if self.opts.reconnect_on_failure {
                        let _ = self.transport.reconnect().await;
                        self.record_reconnect();
                    }
                    self.record_failure(
                        CommsEventKind::Timeout,
                        command.as_deref(),
                        1,
                        started,
                        "read timeout",
                    );
                    return Err(Error::Timeout);
                }
                Err(e) => {
                    self.record_failure(
                        CommsEventKind::ReadFailed,
                        command.as_deref(),
                        1,
                        started,
                        &e.to_string(),
                    );
                    return Err(e);
                }
            }
        }
    }

    fn record_success(
        &self,
        kind: CommsEventKind,
        command: Option<&str>,
        attempt: u32,
        started: Instant,
    ) {
        if let Some(diag) = &self.diagnostics {
            diag.record_success(kind, command, attempt, started.elapsed());
        }
    }

    fn record_failure(
        &self,
        kind: CommsEventKind,
        command: Option<&str>,
        attempt: u32,
        started: Instant,
        detail: &str,
    ) {
        if let Some(diag) = &self.diagnostics {
            diag.record_failure(kind, command, attempt, started.elapsed(), detail);
        }
    }

    fn record_reconnect(&self) {
        if let Some(diag) = &self.diagnostics {
            diag.record_success(CommsEventKind::Reconnect, None, 1, Duration::ZERO);
        }
    }

    fn effective_read_timeout(&self) -> Duration {
        self.opts.per_op_timeout.unwrap_or(self.opts.read_timeout)
    }

    /// Probes and caches whether SYST:ERR? is supported.
    pub async fn probe_syst_err(&mut self) -> bool {
        if let Some(v) = self.capabilities.syst_err {
            return v;
        }
        let supported = self
            .query_with_timeout("SYST:ERR?", Duration::from_millis(500))
            .await
            .is_ok();
        self.capabilities.syst_err = Some(supported);
        supported
    }

    /// Probes and caches whether *OPC? is supported.
    pub async fn probe_opc(&mut self) -> bool {
        if let Some(v) = self.capabilities.opc {
            return v;
        }
        let supported = self
            .query_with_timeout("*OPC?", Duration::from_millis(500))
            .await
            .is_ok();
        self.capabilities.opc = Some(supported);
        supported
    }

    /// Drains the instrument error queue when supported.
    pub async fn check_errors(&mut self) -> Result<Vec<String>> {
        if !self.probe_syst_err().await {
            return Ok(Vec::new());
        }
        let mut errors = Vec::new();
        loop {
            let resp = self.query("SYST:ERR?").await?;
            if resp.starts_with("0,") || resp.starts_with("+0,") {
                break;
            }
            errors.push(resp);
            if errors.len() > 50 {
                break;
            }
        }
        Ok(errors)
    }

    /// Parses a numeric SCPI response.
    pub fn parse_f64(response: &str) -> Result<f64> {
        parse_f64(response)
    }
}
