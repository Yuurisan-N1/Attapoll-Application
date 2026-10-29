use crate::support::cfg::{load_accounts, load_config, load_proxies, Paths};
use crate::support::error::{Error, Result};
use crate::support::log::{lg, lr, ly, mask_proxy, unit_word};

pub fn run(paths: &Paths) -> Result<()> {
    ly("Selftest mode, offline checks only, no network");
    let mut problems = 0u32;

    if load_config(&paths.root).is_err() {
        lr("File config.json was not found");
        problems += 1;
    }

    let proxies = load_proxies(&paths.root);
    if let Some(line) = proxies.first() {
        ly(&format!("Using proxy {}", mask_proxy(line)));
    }

    let data = paths.root.join("data.txt");
    if !data.exists() {
        lr("File data.txt was not found");
        problems += 1;
    } else {
        match load_accounts(&paths.root) {
            Ok(rows) => {
                if rows.iter().any(|row| !row.email.contains('@')) {
                    lr("The email inside data.txt is not a valid address");
                    problems += 1;
                }
            }
            Err(_) => ly("File data.txt is empty so there is no email to sign in with"),
        }
    }

    if problems > 0 {
        lr(&format!(
            "self test found {} {}",
            problems,
            unit_word(problems as u64, "problem", "problems")
        ));
        return Err(Error::State("the self test did not pass".into()));
    }

    lg("Self test passed");
    Ok(())
}
