use crate::error::Result;
use crate::identity::Idn;
use crate::scpi::ScpiSession;
use std::time::Duration;

/// IEEE 488.2 common commands.
pub struct Ieee4882<'a> {
    session: &'a mut ScpiSession,
}

impl<'a> Ieee4882<'a> {
    pub fn new(session: &'a mut ScpiSession) -> Self {
        Self { session }
    }

    pub fn idn(&mut self) -> Result<Idn> {
        let resp = self.session.query("*IDN?")?;
        Ok(Idn::parse(&resp))
    }

    pub fn reset(&mut self) -> Result<()> {
        self.session.write("*RST")?;
        self.wait_complete()
    }

    pub fn clear_status(&mut self) -> Result<()> {
        self.session.write("*CLS")
    }

    pub fn opc_query(&mut self) -> Result<bool> {
        if !self.session.probe_opc() {
            return Ok(true);
        }
        let resp = self.session.query("*OPC?")?;
        Ok(resp.trim() == "1")
    }

    pub fn wait_complete(&mut self) -> Result<()> {
        if self.session.probe_opc() {
            let _ = self
                .session
                .query_with_timeout("*OPC?", Duration::from_secs(30))?;
        }
        Ok(())
    }

    pub fn options(&mut self) -> Result<String> {
        self.session.query("*OPT?")
    }
}

#[cfg(feature = "async")]
use crate::scpi::AsyncScpiSession;

/// Async IEEE 488.2 common commands.
#[cfg(feature = "async")]
pub struct AsyncIeee4882<'a> {
    session: &'a mut AsyncScpiSession,
}

#[cfg(feature = "async")]
impl<'a> AsyncIeee4882<'a> {
    pub fn new(session: &'a mut AsyncScpiSession) -> Self {
        Self { session }
    }

    pub async fn idn(&mut self) -> Result<Idn> {
        let resp = self.session.query("*IDN?").await?;
        Ok(Idn::parse(&resp))
    }

    pub async fn reset(&mut self) -> Result<()> {
        self.session.write("*RST").await?;
        self.wait_complete().await
    }

    pub async fn clear_status(&mut self) -> Result<()> {
        self.session.write("*CLS").await
    }

    pub async fn opc_query(&mut self) -> Result<bool> {
        if !self.session.probe_opc().await {
            return Ok(true);
        }
        let resp = self.session.query("*OPC?").await?;
        Ok(resp.trim() == "1")
    }

    pub async fn wait_complete(&mut self) -> Result<()> {
        if self.session.probe_opc().await {
            let _ = self
                .session
                .query_with_timeout("*OPC?", Duration::from_secs(30))
                .await?;
        }
        Ok(())
    }

    pub async fn options(&mut self) -> Result<String> {
        self.session.query("*OPT?").await
    }
}
