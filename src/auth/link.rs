use crate::auth::email::{self, Credential};
use crate::net::Http;
use crate::support::error::{Error, Result};

pub fn send_magic_link(http: &Http, mail: &str) -> Result<()> {
    email::send_link(http, mail)
}

pub fn sign_in_with_link(http: &Http, mail: &str, raw: &str) -> Result<Credential> {
    let code = email::extract_code(raw);
    if code.is_empty() {
        return Err(Error::Auth("the link carries no sign in code".into()));
    }
    email::exchange(http, mail, &code)
}
