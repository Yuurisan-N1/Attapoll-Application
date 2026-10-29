use crate::net::Http;
use crate::support::error::Result;
use crate::support::log::{lg, ly};

const PATHS: [(&str, &str); 8] = [
    ("access check", "/api/v2/user/access-check?installKey={key}"),
    ("config", "/api/v2/config/feature"),
    ("project list", "/api/v3/user/projects?userId={mail}"),
    ("project history", "/api/v3/user/projects-history"),
    ("gift cards", "/api/v2/user/gift-card-options"),
    ("payment details", "/api/v2/payment/details"),
    ("invite campaign", "/api/v2/user/invite-campaign"),
    (
        "streak terms",
        "/api/v2/config/document/survey-streak-terms",
    ),
];

pub fn sweep(http: &Http, mail: &str) -> Result<()> {
    lg("Probe mode runs read only so nothing is claimed");
    let mut answered = 0usize;
    for (name, template) in PATHS {
        let path = template
            .replace("{key}", http.install_key())
            .replace("{uid}", http.uid())
            .replace("{mail}", mail);
        match http.get(&path) {
            Ok((status, _)) => {
                lg(&format!("Request to {} answered with {}", name, status));
                answered += 1;
            }
            Err(_) => ly(&format!("Request to {} failed with a network error", name)),
        }
    }
    if answered == PATHS.len() {
        lg("The server answered every probe");
    }
    Ok(())
}
