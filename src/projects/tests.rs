use serde_json::json;

use super::response::{section, text};
use super::Projects;

fn sample() -> Projects {
    Projects {
        raw: json!({
            "panelKey": "sg",
            "surveys": [
                {"id": 11, "name": "Daily survey", "reward": 1.25},
                {"id": 12, "title": "Shopping survey", "payout": "0.80", "url": "https://x"}
            ],
            "tasks": [{"id": 21, "name": "Install an app", "reward": 0.5}],
            "games": [],
            "surveyStreak": {"current": 3, "best": 7, "goal": 5}
        }),
    }
}

#[test]
fn every_section_is_counted() {
    let projects = sample();
    let counts = projects.counts();
    let survey = counts
        .iter()
        .find(|(k, _)| *k == "surveys")
        .map(|(_, n)| *n);
    let games = counts.iter().find(|(k, _)| *k == "games").map(|(_, n)| *n);
    assert_eq!(survey, Some(2));
    assert_eq!(games, Some(0));
}

#[test]
fn the_streak_is_read_from_the_list() {
    let projects = sample();
    let streak = crate::streak::progress::Streak::from_projects(&projects);
    assert_eq!(streak.current, 3);
    assert_eq!(streak.best, 7);
}

#[test]
fn text_picks_the_first_key() {
    let item = json!({"name": "A", "title": "B"});
    assert_eq!(text(&item, &["name", "title"]), "A");
    assert_eq!(text(&item, &["missing"]), "");
}

#[test]
fn an_empty_section_is_safe() {
    let raw = json!({});
    assert!(section(&raw, "surveys").is_empty());
    assert!(section(&raw, "tasks").is_empty());
}
