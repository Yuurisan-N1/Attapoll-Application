use serde_json::json;

use crate::net::Http;
use crate::support::error::Result;
use crate::support::log::{lg, ly};

pub fn submit(http: &Http, install_key: &str, nonce: &str, token: &str, gates: bool) -> Result<()> {
    if !gates || token.is_empty() {
        return Ok(());
    }
    let body = json!({
        "installKey": install_key,
        "nonce": nonce,
        "token": token,
        "errorCode": "",
    });
    let (status, _) = http.post("/api/v2/user/device-integrity", &body)?;
    if status == 200 {
        lg("The attestation statement was accepted by the server");
    } else {
        ly("Device telemetry was refused by the server on this run.");
    }
    Ok(())
}
