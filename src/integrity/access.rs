use serde::Deserialize;

use crate::net::Http;
use crate::support::error::Result;
use crate::support::log::lr;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct IntegrityState {
    #[serde(default)]
    pub nonce: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub reason: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AccessCheck {
    #[serde(rename = "ipNotSupported", default)]
    pub ip_blocked: bool,
    #[serde(rename = "countryNotSupported", default)]
    pub country_blocked: bool,
    #[serde(rename = "phoneVerificationRequired", default)]
    pub phone_required: bool,
    #[serde(default)]
    pub integrity: IntegrityState,
}

impl AccessCheck {
    pub fn fetch(http: &Http) -> Result<Self> {
        let (status, value) = http.get(&format!(
            "/api/v2/user/access-check?installKey={}",
            http.install_key()
        ))?;
        if status != 200 {
            return Ok(Self::default());
        }
        let parsed: Self = serde_json::from_value(value).unwrap_or_default();
        if !parsed.usable() {
            lr("Account was refused by the protocol client integrity gate");
        }
        Ok(parsed)
    }

    pub fn usable(&self) -> bool {
        !self.ip_blocked && !self.country_blocked
    }
}
