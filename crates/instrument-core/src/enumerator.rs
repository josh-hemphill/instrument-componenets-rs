use crate::address::ResourceAddress;
use crate::error::Result;
use crate::transport::TransportIdentity;

/// Raw resource from an enumerator before classification.
#[derive(Debug, Clone)]
pub struct RawResource {
    pub address: ResourceAddress,
    pub identity_hint: TransportIdentity,
}

/// Narrow seam for acquiring resources without coupling to VISA.
pub trait ResourceEnumerator: Send + Sync {
    fn list(&self, pattern: &str) -> Result<Vec<RawResource>>;
}

/// Static resource list for unit tests.
#[derive(Debug, Default)]
pub struct StaticEnumerator {
    resources: Vec<RawResource>,
}

impl StaticEnumerator {
    pub fn new(resources: Vec<RawResource>) -> Self {
        Self { resources }
    }

    pub fn from_addresses(addresses: impl IntoIterator<Item = String>) -> Result<Self> {
        let resources = addresses
            .into_iter()
            .map(|raw| {
                Ok(RawResource {
                    address: ResourceAddress::parse(&raw)?,
                    identity_hint: TransportIdentity::default(),
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self::new(resources))
    }
}

impl ResourceEnumerator for StaticEnumerator {
    fn list(&self, pattern: &str) -> Result<Vec<RawResource>> {
        if pattern == "?*INSTR" || pattern == "?*" {
            return Ok(self.resources.clone());
        }
        Ok(self
            .resources
            .iter()
            .filter(|r| r.address.raw.contains(pattern.trim_matches('?')))
            .cloned()
            .collect())
    }
}
