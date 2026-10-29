use crate::projects::response::section;
use crate::projects::Projects;
use crate::support::log::{lg, ly, unit_word};

pub fn report(projects: &Projects) {
    let items = section(&projects.raw, "adjoe");
    if items.is_empty() {
        ly("No rewarded ad was offered to this account");
        return;
    }
    lg(&format!(
        "Adjoe holds {} {} for this account",
        items.len(),
        unit_word(items.len() as u64, "offer", "offers")
    ));
}
