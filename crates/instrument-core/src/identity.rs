use serde::{Deserialize, Serialize};

/// Stable device identity for replacement and reconnection workflows.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceId(pub String);

impl DeviceId {
    /// Builds an ID from manufacturer, model, and serial, falling back to the VISA address.
    pub fn from_identity(identity: &DeviceIdentity, address: &str) -> Self {
        if let (Some(m), Some(model), Some(serial)) = (
            identity.manufacturer.as_deref(),
            identity.model.as_deref(),
            identity.serial.as_deref(),
        ) {
            if !serial.is_empty() {
                return Self(format!("{m}|{model}|{serial}"));
            }
        }
        Self(address.to_string())
    }
}

impl std::fmt::Display for DeviceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Parsed `*IDN?` response.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Idn {
    pub manufacturer: String,
    pub model: String,
    pub serial: String,
    pub firmware: String,
}

impl Idn {
    /// Parses a comma-separated `*IDN?` response.
    pub fn parse(response: &str) -> Self {
        let trimmed = response.trim();
        let parts: Vec<&str> = trimmed.splitn(4, ',').map(str::trim).collect();
        Self {
            manufacturer: parts.first().copied().unwrap_or("").to_string(),
            model: parts.get(1).copied().unwrap_or("").to_string(),
            serial: parts.get(2).copied().unwrap_or("").to_string(),
            firmware: parts.get(3).copied().unwrap_or("").to_string(),
        }
    }

    pub fn format_response(&self) -> String {
        format!(
            "{},{},{},{}",
            self.manufacturer, self.model, self.serial, self.firmware
        )
    }
}

/// Merged device identity from all classification layers.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceIdentity {
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial: Option<String>,
    pub firmware: Option<String>,
    pub options: Option<String>,
}

impl DeviceIdentity {
    pub fn from_idn(idn: &Idn) -> Self {
        Self {
            manufacturer: Some(idn.manufacturer.clone()),
            model: Some(idn.model.clone()),
            serial: Some(idn.serial.clone()),
            firmware: Some(idn.firmware.clone()),
            options: None,
        }
    }

    pub fn merge(&mut self, other: &Self) {
        if self.manufacturer.is_none() {
            self.manufacturer = other.manufacturer.clone();
        }
        if self.model.is_none() {
            self.model = other.model.clone();
        }
        if self.serial.is_none() {
            self.serial = other.serial.clone();
        }
        if self.firmware.is_none() {
            self.firmware = other.firmware.clone();
        }
        if self.options.is_none() {
            self.options = other.options.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_idn() {
        let idn = Idn::parse("Keysight Technologies,34401A,MY123,1.0\n");
        assert_eq!(idn.manufacturer, "Keysight Technologies");
        assert_eq!(idn.model, "34401A");
    }
}
