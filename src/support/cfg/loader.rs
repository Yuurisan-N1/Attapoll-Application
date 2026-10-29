use std::path::{Path, PathBuf};

use crate::support::error::{Error, Result};
use crate::support::log::ly;

use super::schema::Config;

#[derive(Debug, Clone)]
pub struct Paths {
    pub root: PathBuf,
}

impl Paths {
    pub fn detect() -> Self {
        let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self { root }
    }
}

#[derive(Debug, Clone)]
pub struct Account {
    pub email: String,
}

pub fn load_config(root: &Path) -> Result<Config> {
    let path = root.join("config.json");
    match std::fs::read_to_string(&path) {
        Ok(raw) => {
            let cfg: Config = serde_json::from_str(&raw)?;
            cfg.validate().map_err(Error::Cfg)?;
            Ok(cfg)
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            std::fs::write(&path, Config::defaults_text())
                .map_err(|_| Error::Cfg("config.json could not be written".into()))?;
            Ok(Config::default())
        }
        Err(_) => Err(Error::Cfg("config.json could not be read".into())),
    }
}

pub fn load_accounts(root: &Path) -> Result<Vec<Account>> {
    let path = root.join("data.txt");
    let raw = std::fs::read_to_string(&path)
        .map_err(|_| Error::Cfg("File data.txt was not found".into()))?;
    let mut out = Vec::new();
    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let email = line.split('|').next().unwrap_or("").trim();
        if email.is_empty() || !email.contains('@') {
            ly("The email inside data.txt is not a valid address");
            continue;
        }
        out.push(Account {
            email: email.to_string(),
        });
    }
    if out.is_empty() {
        return Err(Error::Cfg(
            "File data.txt is empty so there is no email to sign in with".into(),
        ));
    }
    Ok(out)
}

pub fn load_proxies(root: &Path) -> Vec<String> {
    let path = root.join("proxy.txt");
    let raw = match std::fs::read_to_string(&path) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    raw.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| l.to_string())
        .collect()
}
