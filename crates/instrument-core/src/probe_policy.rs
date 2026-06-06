use serde::{Deserialize, Serialize};

/// How aggressively discovery probes instrument capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ProbePolicy {
    /// Registry, VISA attributes, and *IDN? only — no capability queries.
    None,
    /// Benign read-only state queries (default); does not trigger measurements or change outputs.
    #[default]
    ReadOnly,
    /// Includes acquisition-triggering probes such as `:MEAS:VOLT:DC?`.
    Full,
}
