use serde::{Deserialize, Serialize};

const DEFAULTS: &str = r#"{
  "settings": {
    "sleep_seconds": 3600,
    "retries": 3,
    "gates": false
  },
  "payout": {
    "method": "PAYPAL",
    "account": ""
  }
}
"#;

fn d_sleep() -> u64 {
    3600
}

fn d_retries() -> u32 {
    3
}

fn d_method() -> String {
    "PAYPAL".to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    #[serde(default = "d_sleep")]
    pub sleep_seconds: u64,
    #[serde(default = "d_retries")]
    pub retries: u32,
    #[serde(default)]
    pub gates: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sleep_seconds: d_sleep(),
            retries: d_retries(),
            gates: false,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Payout {
    #[serde(default = "d_method")]
    pub method: String,
    #[serde(default)]
    pub account: String,
}

impl Default for Payout {
    fn default() -> Self {
        Self {
            method: d_method(),
            account: String::new(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub settings: Settings,
    #[serde(default)]
    pub payout: Payout,
}

impl Config {
    pub fn defaults_text() -> &'static str {
        DEFAULTS
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.settings.sleep_seconds < 30 {
            return Err("sleep_seconds must be at least 30".to_string());
        }
        if self.settings.retries == 0 || self.settings.retries > 10 {
            return Err("retries must stay between 1 and 10".to_string());
        }
        if !self.payout.account.is_empty() && !self.payout.account.contains('@') {
            return Err("payout.account must be a mail address".to_string());
        }
        Ok(())
    }

    pub fn sleep_seconds(&self) -> u64 {
        self.settings.sleep_seconds
    }

    pub fn retries(&self) -> u32 {
        self.settings.retries
    }

    pub fn gates(&self) -> bool {
        self.settings.gates
    }

    pub fn payout_account(&self) -> &str {
        &self.payout.account
    }

    pub fn payout_method(&self) -> &str {
        &self.payout.method
    }
}
