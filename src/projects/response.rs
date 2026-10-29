use serde_json::Value;

pub const SECTIONS: [&str; 7] = [
    "surveys",
    "tasks",
    "games",
    "adjoe",
    "inMarket",
    "promotions",
    "recommendations",
];

pub fn text(value: &Value, keys: &[&str]) -> String {
    for key in keys {
        if let Some(found) = value.get(*key) {
            match found {
                Value::String(s) if !s.is_empty() => return s.clone(),
                Value::Number(n) => return n.to_string(),
                _ => {}
            }
        }
    }
    String::new()
}

pub fn section<'a>(root: &'a Value, key: &str) -> Vec<&'a Value> {
    match root.get(key) {
        Some(Value::Array(items)) => items.iter().collect(),
        Some(Value::Object(_)) => root.get(key).map(|v| vec![v]).unwrap_or_default(),
        _ => Vec::new(),
    }
}
