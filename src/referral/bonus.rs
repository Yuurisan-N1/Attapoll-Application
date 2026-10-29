use serde_json::json;

use crate::net::Http;
use crate::support::error::Result;
use crate::support::log::{lg, ly, unit_word};

pub fn claim(http: &Http) -> Result<()> {
    let body = json!({});
    let (status, _) = http.post("/api/v2/user/referral/claim-bonus", &body)?;
    if status != 200 {
        ly("No referral bonus is ready to collect yet");
    }
    Ok(())
}

pub fn earnings(http: &Http) -> Result<()> {
    let (status, value) = http.get("/api/v2/user/referral/earnings-details")?;
    if status != 200 {
        ly("No referral commission was pending on this account");
        return Ok(());
    }
    let entries = value.as_array().map(|v| v.len()).unwrap_or(0);
    if entries == 0 {
        ly("No referral commission was pending on this account");
        return Ok(());
    }
    lg(&format!(
        "Referral list holds {} {} for this account",
        entries,
        unit_word(entries as u64, "entry", "entries")
    ));
    Ok(())
}
