mod auth;
mod cashout;
mod cli;
mod integrity;
mod net;
mod offerwall;
mod projects;
mod referral;
mod run;
mod streak;
mod support;

fn watch_interrupt() {
    std::thread::spawn(|| {
        let rt = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => rt,
            Err(_) => return,
        };
        rt.block_on(async {
            if tokio::signal::ctrl_c().await.is_ok() {
                support::log::restore_echo();
                support::log::newline();
                support::log::lr("Script stopped by user");
                std::process::exit(0);
            }
        });
    });
}

fn main() {
    support::log::install();
    support::banner::print_banner();
    watch_interrupt();

    if let Err(e) = cli::dispatch::run() {
        support::log::lr(&format!(
            "The bot stopped because {}",
            support::log::explain(&e)
        ));
        std::process::exit(1);
    }
}
