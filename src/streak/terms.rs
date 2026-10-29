use crate::net::Http;

const DOC: &str = "survey-streak-terms";

pub fn fetch(http: &Http) {
    let path = format!("/api/v2/config/document/{}", DOC);
    let _ = http.get(&path);
}
