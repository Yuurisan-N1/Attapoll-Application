use serde_json::Value;

use crate::projects::response::text;
use crate::projects::Projects;
use crate::support::log::{lg, ly};

pub struct Streak {
    pub current: u32,
    pub best: u32,
    pub goal: u32,
}

impl Streak {
    pub fn from_projects(projects: &Projects) -> Self {
        let raw = projects
            .raw
            .get("surveyStreak")
            .cloned()
            .unwrap_or(Value::Null);
        let current = number(&raw, &["current", "currentStreak", "count", "days"]);
        let best = number(&raw, &["best", "longest", "bestStreak", "max"]);
        let goal = number(&raw, &["goal", "target", "required", "next"]);
        Self {
            current,
            best,
            goal,
        }
    }

    pub fn report(&self) {
        if self.current == 0 && self.best == 0 && self.goal == 0 {
            ly("Daily streak is not due on this account yet");
            return;
        }
        lg(&format!(
            "Daily streak is now day {} on this account",
            self.current
        ));
    }
}

fn number(value: &Value, keys: &[&str]) -> u32 {
    if let Ok(raw) = text(value, keys).parse::<f64>() {
        return raw as u32;
    }
    match value {
        Value::Array(items) => items.len() as u32,
        _ => 0,
    }
}
