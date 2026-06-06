use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Receives push notifications for instrument communication events.
pub trait CommsObserver: Send + Sync {
    fn on_event(&self, event: &CommsEvent);
}

/// Kind of communication event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CommsEventKind {
    WriteOk,
    WriteFailed,
    ReadOk,
    ReadFailed,
    Timeout,
    Reconnect,
}

/// A single communication event for observer push and tracing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CommsEvent {
    pub address: String,
    pub kind: CommsEventKind,
    pub command: Option<String>,
    pub attempt: u32,
    pub elapsed_ms: u64,
    pub detail: Option<String>,
}

/// Pollable health snapshot for a device address.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceHealth {
    pub consecutive_failures: u32,
    pub total_operations: u64,
    pub total_failures: u64,
    pub last_error: Option<String>,
    pub last_success_unix_ms: Option<u64>,
    pub last_failure_unix_ms: Option<u64>,
}

impl DeviceHealth {
    pub fn is_healthy(&self) -> bool {
        self.consecutive_failures == 0
    }
}

/// Shared diagnostics context injected into SCPI sessions.
#[derive(Clone)]
pub struct Diagnostics {
    address: String,
    health: Option<Arc<Mutex<DeviceHealth>>>,
    observer: Option<Arc<dyn CommsObserver>>,
}

impl Diagnostics {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            health: None,
            observer: None,
        }
    }

    pub fn with_health(mut self, health: Arc<Mutex<DeviceHealth>>) -> Self {
        self.health = Some(health);
        self
    }

    pub fn with_observer(mut self, observer: Arc<dyn CommsObserver>) -> Self {
        self.observer = Some(observer);
        self
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn record_success(
        &self,
        kind: CommsEventKind,
        command: Option<&str>,
        attempt: u32,
        elapsed: Duration,
    ) {
        if let Some(health) = &self.health {
            let mut h = health.lock().unwrap();
            h.consecutive_failures = 0;
            h.total_operations += 1;
            h.last_success_unix_ms = Some(now_unix_ms());
        }
        self.emit(kind, command, attempt, elapsed, None);
    }

    pub fn record_failure(
        &self,
        kind: CommsEventKind,
        command: Option<&str>,
        attempt: u32,
        elapsed: Duration,
        detail: &str,
    ) {
        if let Some(health) = &self.health {
            let mut h = health.lock().unwrap();
            h.consecutive_failures += 1;
            h.total_operations += 1;
            h.total_failures += 1;
            h.last_error = Some(detail.to_string());
            h.last_failure_unix_ms = Some(now_unix_ms());
        }
        self.emit(kind, command, attempt, elapsed, Some(detail.to_string()));
    }

    fn emit(
        &self,
        kind: CommsEventKind,
        command: Option<&str>,
        attempt: u32,
        elapsed: Duration,
        detail: Option<String>,
    ) {
        let should_trace = tracing::enabled!(tracing::Level::DEBUG);
        let should_push = self.observer.is_some();
        if !should_trace && !should_push {
            return;
        }

        let event = CommsEvent {
            address: self.address.clone(),
            kind,
            command: command.map(str::to_string),
            attempt,
            elapsed_ms: elapsed.as_millis() as u64,
            detail,
        };

        if should_trace {
            match kind {
                CommsEventKind::WriteFailed
                | CommsEventKind::ReadFailed
                | CommsEventKind::Timeout => tracing::warn!(
                    address = %event.address,
                    command = ?event.command,
                    attempt = event.attempt,
                    detail = ?event.detail,
                    "instrument comms failure"
                ),
                _ => tracing::debug!(
                    address = %event.address,
                    command = ?event.command,
                    kind = ?event.kind,
                    "instrument comms"
                ),
            }
        }

        if let Some(observer) = &self.observer {
            observer.on_event(&event);
        }
    }
}

fn now_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
