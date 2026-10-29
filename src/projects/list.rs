use serde_json::Value;

use crate::net::Http;
use crate::support::error::{Error, Result};
use crate::support::log::{lg, unit_word};

use super::response::{section, SECTIONS};

pub struct Projects {
    pub raw: Value,
}

impl Projects {
    pub fn fetch(http: &Http, mail: &str) -> Result<Self> {
        let (status, value) = http.get_query("/api/v3/user/projects", &[("userId", mail)])?;
        if status != 200 {
            let reason = value
                .pointer("/error/errors/0/message")
                .and_then(|v| v.as_str())
                .unwrap_or("the project list was refused");
            if status == 401 || status == 403 {
                return Err(Error::Gate(format!("{} {}", status, reason)));
            }
            return Err(Error::Api {
                status,
                body: reason.to_string(),
            });
        }
        Ok(Self { raw: value })
    }

    pub fn counts(&self) -> Vec<(&'static str, usize)> {
        SECTIONS
            .iter()
            .map(|key| (*key, section(&self.raw, key).len()))
            .collect()
    }

    pub fn report(&self) {
        for (name, total) in self.counts() {
            if total == 0 {
                continue;
            }
            lg(&format!(
                "The {} list holds {} {} for this account",
                name,
                total,
                unit_word(total as u64, "project", "projects")
            ));
        }
    }
}
