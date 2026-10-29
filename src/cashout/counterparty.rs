use serde_json::json;

use crate::net::Http;
use crate::support::error::Result;
use crate::support::log::lg;

pub fn fetch(http: &Http, kind: &str, value: &str) -> Result<()> {
    let path = format!(
        "/api/v2/payment/fetch-counterparty?type={}&value={}",
        kind, value
    );
    let (status, payload) = http.get(&path)?;
    if status != 200 {
        return Ok(());
    }
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("");
    if !name.is_empty() {
        lg(&format!("The payout recipient {} was found", name));
    }
    Ok(())
}

pub fn save(http: &Http, kind: &str, value: &str) -> Result<()> {
    let body = json!({ "type": kind, "value": value });
    let (status, _) = http.post("/api/v2/payment/counterparty", &body)?;
    if status == 200 {
        lg("Payout wallet was saved for this account");
    }
    Ok(())
}
