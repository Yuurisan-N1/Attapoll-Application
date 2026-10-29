use serde_json::Value;

use crate::net::Http;
use crate::support::error::Result;
use crate::support::log::{lg, ly, unit_word};

pub fn project_activity(http: &Http) -> Result<()> {
    let (status, value) = http.get("/api/v2/user/project-activity")?;
    if status != 200 {
        ly("The project activity state could not be read for this account");
        return Ok(());
    }
    let open = value
        .get("activity")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    if open {
        lg("The project activity window is open for this account");
    } else {
        ly("The project activity window is closed for this account");
    }
    Ok(())
}

pub fn summary(http: &Http) -> Result<()> {
    let (status, value) = http.get("/api/v2/user-activity/summary")?;
    if status != 200 {
        ly("The earning summary could not be read for this account");
        return Ok(());
    }
    let rows = value.as_array().cloned().unwrap_or_default();
    if rows.is_empty() {
        ly("The earning summary holds no entry for this account");
        return Ok(());
    }
    lg(&format!(
        "The earning summary holds {} {} for this account",
        rows.len(),
        unit_word(rows.len() as u64, "entry", "entries")
    ));
    for row in rows.iter().take(5) {
        let line = row_line(row);
        if !line.is_empty() {
            lg(&line);
        }
    }
    Ok(())
}

fn row_line(row: &Value) -> String {
    let Some(amount) = row.get("amount").map(text_of) else {
        return String::new();
    };
    let Some(currency) = row.get("currency").map(text_of) else {
        return String::new();
    };
    let label = row
        .get("description")
        .or_else(|| row.get("type"))
        .map(text_of)
        .unwrap_or_else(|| String::from("entry"));
    format!("{} paid {} {}", label, amount, currency)
}

fn text_of(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        _ => String::new(),
    }
}
