use serde_json::{json, Value};

use crate::net::Http;
use crate::support::error::Result;
use crate::support::log::{lg, ly, unit_word};

pub fn report(http: &Http) -> Result<()> {
    let (status, value) = http.get("/api/v2/survey/initial-survey")?;
    if status != 200 {
        ly("The first survey could not be read for this account");
        return Ok(());
    }
    let questions = value
        .get("questions")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    if questions.is_empty() {
        lg("The first survey of this account was already answered");
        return Ok(());
    }
    let title = value
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("the first survey");
    lg(&format!(
        "{} holds {} {} for this account",
        title,
        questions.len(),
        unit_word(questions.len() as u64, "question", "questions")
    ));
    Ok(())
}

fn stamp(offset_secs: i64) -> String {
    let now = chrono::Utc::now() + chrono::Duration::seconds(offset_secs);
    now.format("%Y-%m-%dT%H:%M:%S.000Z").to_string()
}

fn answers(questions: &Value) -> Vec<Value> {
    let mut rows = Vec::new();
    let list = match questions.as_array() {
        Some(list) => list,
        None => return rows,
    };
    for question in list {
        let qid = match question.get("questionId") {
            Some(v) => v.clone(),
            None => continue,
        };
        let option = question
            .get("options")
            .and_then(|v| v.as_array())
            .and_then(|v| v.first())
            .and_then(|v| v.get("optionId"))
            .cloned();
        let choice = match option {
            Some(v) => v,
            None => continue,
        };
        rows.push(json!({ "questionId": qid, "answerChoiceId": choice }));
    }
    rows
}

fn submit(http: &Http, survey: &Value, rows: &[Value]) -> Option<(f64, bool)> {
    let sid = survey.get("id")?.as_str()?.to_string();
    let kind = survey
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("Profiling")
        .to_string();
    let body = json!({
        "installKey": http.install_key(),
        "locationAccuracy": 0.0,
        "responseAnswers": rows,
        "responseExtras": "{}",
        "responseLatitude": 0.0,
        "responseLongitude": 0.0,
        "responseStartTime": stamp(-420),
        "responseTime": stamp(0),
        "responseType": kind,
        "surveyId": sid,
        "syncStatus": "Y",
    });
    let (status, value) = http
        .post("/_ah/api/surveyApi/v1/insertUserResponse", &body)
        .ok()?;
    if status != 200 {
        return None;
    }
    let credits = value
        .get("creditsEarned")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let trusted = value
        .get("trusted")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    Some((credits, trusted))
}

pub fn answer(http: &Http, projects: &Value) -> Result<()> {
    let surveys = match projects.get("surveys").and_then(|v| v.as_array()) {
        Some(list) => list.clone(),
        None => return Ok(()),
    };
    if surveys.is_empty() {
        return Ok(());
    }
    let mut sent = 0usize;
    let mut credits = 0.0f64;
    let mut trusted = 0usize;
    for survey in &surveys {
        let rows = answers(survey.get("questions").unwrap_or(&Value::Null));
        if rows.is_empty() {
            continue;
        }
        if let Some((earned, ok)) = submit(http, survey, &rows) {
            sent += 1;
            credits += earned;
            if ok {
                trusted += 1;
            }
        }
    }
    if sent == 0 {
        ly("The survey answers could not be sent for this account");
        return Ok(());
    }
    lg(&format!(
        "The bot answered {} {} for this account",
        sent,
        unit_word(sent as u64, "survey", "surveys")
    ));
    if credits > 0.0 {
        lg(&format!(
            "The server credited {:.2} USD for these answers",
            credits
        ));
    } else if trusted == 0 {
        ly("The server did not credit these answers for this account");
    }
    Ok(())
}
