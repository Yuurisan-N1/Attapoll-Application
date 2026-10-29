use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::support::consts::app;
use crate::support::consts::net as hn;

pub fn base(install_key: &str, aid: &str) -> HeaderMap {
    let mut map = HeaderMap::new();
    let pairs = [
        (hn::H_PACKAGE, app::PACKAGE),
        (hn::H_CERT, app::CERT_SHA1),
        (hn::H_AID, aid),
        (hn::H_INSTALL, install_key),
        (hn::H_LOCALE, app::LOCALE),
        (hn::H_OS, app::OS_VERSION),
        (hn::H_PLATFORM, app::PLATFORM),
        (hn::H_TZ, app::TZ),
        (hn::H_VERSION, app::APP_VERSION),
        (hn::H_VPN, "false"),
    ];
    for (name, value) in pairs {
        if let (Ok(n), Ok(v)) = (
            HeaderName::from_bytes(name.as_bytes()),
            HeaderValue::from_str(value),
        ) {
            map.insert(n, v);
        }
    }
    map
}

pub fn android_only() -> HeaderMap {
    let mut map = HeaderMap::new();
    let pairs = [(hn::H_PACKAGE, app::PACKAGE), (hn::H_CERT, app::CERT_SHA1)];
    for (name, value) in pairs {
        if let (Ok(n), Ok(v)) = (
            HeaderName::from_bytes(name.as_bytes()),
            HeaderValue::from_str(value),
        ) {
            map.insert(n, v);
        }
    }
    map
}
