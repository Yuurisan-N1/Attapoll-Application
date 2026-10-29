use crate::net::Http;
use crate::support::log::{lg, ly};

pub fn summary(http: &Http) {
    match http.get("/api/v2/user/balance") {
        Ok((status, value)) => {
            if status != 200 {
                ly(&format!(
                    "Balance was refused by the server with {}",
                    status
                ));
                return;
            }
            let amount = value
                .get("currentBalance")
                .or_else(|| value.get("balance"))
                .or_else(|| value.get("amt"))
                .map(|v| v.to_string().trim_matches('"').to_string())
                .unwrap_or_default();
            if amount.is_empty() {
                ly("Balance could not be read for this account");
                return;
            }
            lg(&format!("Balance holds {} USD for this account", amount));
        }
        Err(_) => ly("Balance could not be read for this account"),
    }
}
