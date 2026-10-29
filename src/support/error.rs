use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("the local files are wrong: {0}")]
    Cfg(String),
    #[error("the file could not be read: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Http(String),
    #[error("the reply could not be read: {0}")]
    Json(#[from] serde_json::Error),
    #[error("the api answered {status}")]
    Api { status: u16, body: String },
    #[error("the sign in was refused: {0}")]
    Auth(String),
    #[error("the account is held back: {0}")]
    Gate(String),
    #[error("the run could not continue: {0}")]
    State(String),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(net_reason(&err))
    }
}

fn net_reason(err: &reqwest::Error) -> String {
    if err.is_timeout() {
        "the request timed out".to_string()
    } else if err.is_connect() {
        "the connection could not be established".to_string()
    } else if err.is_body() || err.is_decode() {
        "the reply could not be read".to_string()
    } else if let Some(status) = err.status() {
        format!("the server answered {}", status.as_u16())
    } else if err.is_redirect() {
        "the request was sent to another address".to_string()
    } else {
        "the request failed before an answer arrived".to_string()
    }
}

impl Error {
    pub fn refusal(code: &str) -> String {
        match code {
            "EMAIL_NOT_FOUND" => "the mail is not registered yet".to_string(),
            "INVALID_EMAIL" | "MISSING_EMAIL" => "the mail address is not accepted".to_string(),
            "INVALID_OOB_CODE" => "the sign in code is not valid".to_string(),
            "EXPIRED_OOB_CODE" => "the sign in code is too old".to_string(),
            "INVALID_ID_TOKEN" => "the stored token is not valid".to_string(),
            "TOO_MANY_ATTEMPTS_TRY_LATER" => {
                "the server wants a pause before the next try".to_string()
            }
            "USER_DISABLED" => "the account is disabled for now".to_string(),
            "OPERATION_NOT_ALLOWED" => {
                "the mail link sign in is turned off for this project".to_string()
            }
            _ => "the server refused the request".to_string(),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
