use serde_json::json;

use crate::net::Http;
use crate::support::error::Result;
use crate::support::log::{lg, ly};

pub fn apply(http: &Http, code: &str) -> Result<()> {
    if code.trim().is_empty() {
        ly("The inviter was skipped because no code was configured");
        return Ok(());
    }
    let body = json!({ "code": code.trim() });
    let (status, _) = http.post("/api/v2/user/referral/accept-offer", &body)?;
    if status == 200 {
        lg("Referral activation is confirmed for this account");
        return Ok(());
    }
    ly(&format!("Referral bind was refused with {}", status));
    Ok(())
}

pub fn campaign(http: &Http) {
    match http.get("/api/v2/user/invite-campaign") {
        Ok((status, _)) => {
            if status != 200 {
                ly("Referral activation is not due on this account");
            }
        }
        Err(_) => ly("Referral activation is not due on this account"),
    }
}
