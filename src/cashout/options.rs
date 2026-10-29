use serde_json::Value;

use crate::net::Http;
use crate::support::error::Result;
use crate::support::log::{lg, ly, unit_word};

pub fn payment(http: &Http) -> Result<()> {
    let (status, value) = http.get("/api/v2/user/payment-options")?;
    if status != 200 {
        ly("The payment options could not be read for this account");
        return Ok(());
    }
    let rows = value
        .get("paymentOptionList")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    if rows.is_empty() {
        ly("The payment options list is empty for this account");
        return Ok(());
    }
    lg(&format!(
        "The payment options list holds {} {} for this account",
        rows.len(),
        unit_word(rows.len() as u64, "route", "routes")
    ));
    for row in rows.iter().take(5) {
        let line = option_line(row);
        if !line.is_empty() {
            lg(&line);
        }
    }
    Ok(())
}

fn option_line(row: &Value) -> String {
    let name = row
        .get("optionName")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    if name.is_empty() {
        return String::new();
    }
    let minimum = row.get("minimumAmount").map(text_of).unwrap_or_default();
    if minimum.is_empty() {
        return String::new();
    }
    let currency = row
        .get("currency")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    if currency.is_empty() {
        return format!("{} payouts start at {}", name, minimum);
    }
    format!("{} payouts start at {} {}", name, minimum, currency)
}

fn text_of(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        _ => String::new(),
    }
}
