use crate::net::Egress;
use crate::run::account;
use crate::support::cfg::{load_accounts, load_proxies, Config, Paths};
use crate::support::error::Result;
use crate::support::log::{self, lg, lr, ly};

pub fn once(cfg: &Config, paths: &Paths) -> Result<()> {
    let accounts = load_accounts(&paths.root)?;
    let mut egress = Egress::new(load_proxies(&paths.root));
    let total = accounts.len();
    let mut cleared = 0usize;

    for (index, account) in accounts.iter().enumerate() {
        if index > 0 {
            log::gap();
        }
        let slot = index + 1;
        let proxy = egress.next();
        ly(&format!("Account {} started the cycle", slot));
        if let Some(line) = proxy.as_deref() {
            ly(&format!("Using proxy {}", log::mask_proxy(line)));
        }
        match account::run(cfg, paths, account, proxy, slot) {
            Ok(outcome) => {
                cleared += 1;
                lg(&format!(
                    "Account {} holds {} {} at the end of the cycle",
                    slot,
                    outcome.projects,
                    log::unit_word(outcome.projects as u64, "project", "projects")
                ));
            }
            Err(e) => lr(&format!(
                "Account {} was skipped because {}",
                slot,
                log::explain(&e)
            )),
        }
    }

    let _ = (cleared, total);
    Ok(())
}

pub fn start(cfg: &Config, paths: &Paths) -> Result<()> {
    let mut number = 1u64;
    loop {
        ly(&format!("Starting automation cycle number {}", number));
        match once(cfg, paths) {
            Ok(()) => lg(&format!("Automation cycle number {} is complete", number)),
            Err(e) => lr(&format!("The cycle stopped because {}", log::explain(&e))),
        }
        log::countdown(cfg.sleep_seconds(), "Next cycle starts in");
        number += 1;
    }
}
