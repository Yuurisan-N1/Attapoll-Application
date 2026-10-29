use serde_json::json;

use crate::net::Http;
use crate::support::error::{Error, Result};
use crate::support::log::{lg, money};

pub struct Payout {
    pub amount: f64,
    pub currency: String,
    pub payment_type: String,
    pub counterparty: String,
    pub name: String,
    pub address: String,
}

impl Payout {
    pub fn to(amount: f64, method: &str, target: &str) -> Self {
        Self {
            amount,
            currency: "USD".to_string(),
            payment_type: method.to_string(),
            counterparty: target.to_string(),
            name: String::new(),
            address: String::new(),
        }
    }

    pub fn valid(&self) -> bool {
        self.amount > 0.0 && !self.counterparty.is_empty() && !self.payment_type.is_empty()
    }
}

pub fn submit(http: &Http, user_id: &str, install_key: &str, payout: &Payout) -> Result<()> {
    if !payout.valid() {
        return Err(Error::Gate("the payout details are not complete".into()));
    }
    let body = json!({
        "userId": user_id,
        "installKey": install_key,
        "amt": payout.amount,
        "ccy": payout.currency,
        "counterPartyId": payout.counterparty,
        "paymentType": payout.payment_type,
        "addresseeName": payout.name,
        "paymentAddress": payout.address,
        "paymentCurrencyAmount": payout.amount,
    });
    let (status, value) = http.post("/api/v2/payment/request", &body)?;
    if status != 200 {
        return Err(Error::Api {
            status,
            body: "the payout was refused".to_string(),
        });
    }
    let id = value
        .get("id")
        .or_else(|| value.get("paymentId"))
        .map(|v| v.to_string());
    match id {
        Some(id) => lg(&format!(
            "The payout of {} {} was requested under id {}",
            money(&format!("{:.2}", payout.amount)),
            payout.currency,
            id
        )),
        None => lg(&format!(
            "The payout of {} {} was requested",
            money(&format!("{:.2}", payout.amount)),
            payout.currency
        )),
    }
    Ok(())
}
