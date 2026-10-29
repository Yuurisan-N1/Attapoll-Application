use serde_json::json;

use crate::net::Http;
use crate::projects::response::section;
use crate::projects::Projects;
use crate::support::error::Result;
use crate::support::log::{lg, ly, unit_word};

pub fn report(projects: &Projects) {
    let items = section(&projects.raw, "inMarket");
    if items.is_empty() {
        return;
    }
    lg(&format!(
        "In market holds {} {} for this account",
        items.len(),
        unit_word(items.len() as u64, "offer", "offers")
    ));
}

pub fn hide(http: &Http, offer_id: &str) -> Result<()> {
    if offer_id.is_empty() {
        ly("The offer id is empty so nothing was hidden");
        return Ok(());
    }
    let body = json!({ "offerId": offer_id });
    let (status, _) = http.post("/api/v2/user/offers/stop-showing", &body)?;
    if status != 200 {
        let _ = status;
        ly("The offer was refused by the server");
    }
    Ok(())
}
