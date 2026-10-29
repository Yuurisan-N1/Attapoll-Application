use serde_json::Value;

use crate::net::Http;
use crate::support::error::{Error, Result};
use crate::support::log::{lg, unit_word};

pub fn fetch(http: &Http) -> Result<Vec<Value>> {
    let (status, value) = http.get("/api/v3/user/projects-history")?;
    if status != 200 {
        return Err(Error::Api {
            status,
            body: "the history was refused".to_string(),
        });
    }
    let rows = match value {
        Value::Array(items) => items,
        Value::Object(ref map) => map
            .get("history")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default(),
        _ => Vec::new(),
    };
    lg(&format!(
        "The project history holds {} finished {} for this account",
        rows.len(),
        unit_word(rows.len() as u64, "project", "projects")
    ));
    Ok(rows)
}
