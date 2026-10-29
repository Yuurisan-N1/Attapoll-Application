use uuid::Uuid;

use crate::auth::session::Session;
use crate::support::log::lg;

#[derive(Debug, Clone, Default)]
pub struct Device {
    pub install_key: String,
    pub aid: String,
}

impl Device {
    pub fn ensure(session: &mut Session) -> Self {
        if session.install_key.is_empty() {
            session.install_key = Uuid::new_v4().to_string();
            lg("A new device identity was created and cached for this account");
        }
        if !is_uuid(&session.aid) {
            session.aid = Uuid::new_v4().to_string();
        }
        Self {
            install_key: session.install_key.clone(),
            aid: session.aid.clone(),
        }
    }
}

fn is_uuid(value: &str) -> bool {
    Uuid::parse_str(value).is_ok()
}
