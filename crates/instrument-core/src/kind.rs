use serde::{Deserialize, Serialize};

/// Instrument functional class (IVI-inspired).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InstrumentKind {
    Dmm,
    DcPowerSupply,
    FunctionGenerator,
    Unknown,
}

impl InstrumentKind {
    pub fn from_str_label(label: &str) -> Option<Self> {
        match label {
            "Dmm" => Some(Self::Dmm),
            "DcPowerSupply" => Some(Self::DcPowerSupply),
            "FunctionGenerator" => Some(Self::FunctionGenerator),
            "Unknown" => Some(Self::Unknown),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Dmm => "Dmm",
            Self::DcPowerSupply => "DcPowerSupply",
            Self::FunctionGenerator => "FunctionGenerator",
            Self::Unknown => "Unknown",
        }
    }
}

impl std::fmt::Display for InstrumentKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
