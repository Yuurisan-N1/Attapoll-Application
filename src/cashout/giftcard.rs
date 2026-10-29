use crate::net::Http;
use crate::support::log::{lg, ly, unit_word};

pub fn report(http: &Http) {
    match http.get("/api/v2/user/gift-card-options") {
        Ok((status, value)) => {
            if status != 200 {
                ly("Payout history was not returned by the server");
                return;
            }
            let total = value.as_array().map(|v| v.len()).unwrap_or(0);
            if total == 0 {
                return;
            }
            lg(&format!(
                "Gift card list holds {} {} for this account",
                total,
                unit_word(total as u64, "card", "cards")
            ));
        }
        Err(_) => ly("Payout history was not returned by the server"),
    }
}
