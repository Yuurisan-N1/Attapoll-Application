use crate::net::Http;
use crate::support::error::Result;
use crate::support::log::{lg, ly, unit_word};

pub fn messages(http: &Http) -> Result<()> {
    let (status, value) = http.get("/api/v2/user/messages")?;
    if status != 200 {
        ly("The inbox could not be read for this account");
        return Ok(());
    }
    let rows = value.as_array().cloned().unwrap_or_default();
    if rows.is_empty() {
        lg("The inbox holds no message for this account");
        return Ok(());
    }
    lg(&format!(
        "The inbox holds {} {} for this account",
        rows.len(),
        unit_word(rows.len() as u64, "message", "messages")
    ));
    Ok(())
}

pub fn categories(http: &Http) -> Result<()> {
    let (status, value) = http.get("/api/v2/user/profile-categories")?;
    if status != 200 {
        ly("The profile categories could not be read for this account");
        return Ok(());
    }
    let rows = value.as_array().cloned().unwrap_or_default();
    if rows.is_empty() {
        lg("The profile categories are already complete for this account");
        return Ok(());
    }
    lg(&format!(
        "The profile holds {} {} left to answer",
        rows.len(),
        unit_word(rows.len() as u64, "category", "categories")
    ));
    Ok(())
}
