use serde_json::json;

use crate::net::Http;
use crate::support::error::{Error, Result};

pub struct Credential {
    pub id_token: String,
    pub refresh_token: String,
    pub uid: String,
    pub verified: bool,
}

pub fn send_link(http: &Http, email: &str) -> Result<()> {
    let body = json!({
        "requestType": "EMAIL_SIGNIN",
        "email": email,
        "continueUrl": "https://earn.attapoll.app/",
    });
    let (status, value) = http.firebase("accounts:sendOobCode", &body)?;
    if status != 200 {
        return Err(Error::Auth(Error::refusal(code_of(&value))));
    }
    Ok(())
}

pub fn exchange(http: &Http, email: &str, oob_code: &str) -> Result<Credential> {
    let body = json!({
        "email": email,
        "oobCode": oob_code,
        "returnSecureToken": true,
    });
    let (status, value) = http.firebase("accounts:signInWithEmailLink", &body)?;
    if status != 200 {
        return Err(Error::Auth(Error::refusal(code_of(&value))));
    }
    parse(value)
}

pub fn update_profile(http: &Http, id_token: &str, display: &str) -> Result<()> {
    let body = json!({
        "idToken": id_token,
        "displayName": display,
        "returnSecureToken": true,
    });
    let (status, _) = http.firebase("accounts:update", &body)?;
    let _ = status;
    Ok(())
}

pub fn extract_code(raw: &str) -> String {
    let trimmed = raw.trim();
    let marker = "oobCode=";
    if let Some(at) = trimmed.find(marker) {
        let rest = &trimmed[at + marker.len()..];
        let end = rest.find(['&', '#', ' ', '\n']).unwrap_or(rest.len());
        return rest[..end].to_string();
    }
    if trimmed.contains(' ') || trimmed.contains('/') {
        return String::new();
    }
    trimmed.to_string()
}

fn code_of(value: &serde_json::Value) -> &str {
    value
        .get("error")
        .and_then(|e| e.get("message"))
        .and_then(|m| m.as_str())
        .unwrap_or("")
}

fn parse(value: serde_json::Value) -> Result<Credential> {
    let id_token = value
        .get("idToken")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::Auth("the reply carried no token".into()))?
        .to_string();
    let refresh_token = value
        .get("refreshToken")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let uid = value
        .get("localId")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let verified = value
        .get("emailVerified")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    Ok(Credential {
        id_token,
        refresh_token,
        uid,
        verified,
    })
}
