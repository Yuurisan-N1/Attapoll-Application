use crate::support::error::Error;

const G: &str = "\x1b[1;32m";
const Y: &str = "\x1b[1;33m";
const R: &str = "\x1b[1;31m";
const Z: &str = "\x1b[0m";

pub const LINE_MAX: usize = 96;

static LINES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[allow(dead_code)]
pub fn lines_written() -> usize {
    LINES.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn install() {
    if cfg!(target_os = "windows") {
        let _ = std::process::Command::new("cmd")
            .args(["/c", "chcp 65001"])
            .status();
    }
}

fn line(color: &str, msg: &str) {
    LINES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    println!("{}{}{}", color, clamp(msg), Z);
    let _ = std::io::Write::flush(&mut std::io::stdout());
}

pub fn lg(msg: &str) {
    line(G, msg);
}

pub fn ly(msg: &str) {
    line(Y, msg);
}

pub fn lr(msg: &str) {
    line(R, msg);
}

static LIVE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

static ASKING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn blank() {
    let live = LIVE.swap(false, std::sync::atomic::Ordering::Relaxed);
    let asking = ASKING.load(std::sync::atomic::Ordering::Relaxed);
    if live {
        print!("\r\x1b[2K\r");
    } else if asking {
        println!();
    }
    let _ = std::io::Write::flush(&mut std::io::stdout());
}

pub fn clock() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[allow(dead_code)]
pub fn ask(msg: &str) -> String {
    blank();
    print!("{}{}{} ", Y, squeeze(msg), Z);
    let _ = std::io::Write::flush(&mut std::io::stdout());

    let mut answer = String::new();
    ASKING.store(true, std::sync::atomic::Ordering::Relaxed);
    let _ = std::io::stdin().read_line(&mut answer);
    ASKING.store(false, std::sync::atomic::Ordering::Relaxed);
    {
        use std::io::IsTerminal;
        if !std::io::stdin().is_terminal() {
            println!();
        }
    }
    answer.trim().to_string()
}

#[allow(dead_code)]
pub fn ask_secret(msg: &str) -> String {
    blank();
    let label = squeeze(msg);
    let echo = EchoGuard::off();
    print!("{}{}{} ", Y, label, Z);
    let _ = std::io::Write::flush(&mut std::io::stdout());

    let mut answer = String::new();
    ASKING.store(true, std::sync::atomic::Ordering::Relaxed);
    let _ = std::io::stdin().read_line(&mut answer);
    ASKING.store(false, std::sync::atomic::Ordering::Relaxed);
    drop(echo);

    let trimmed = answer.trim().to_string();
    println!("{}{}{}", G, "*".repeat(10), Z);
    let _ = std::io::Write::flush(&mut std::io::stdout());
    trimmed
}

struct EchoGuard {
    suppressed: bool,
}

impl EchoGuard {
    fn off() -> EchoGuard {
        EchoGuard {
            suppressed: set_echo(false),
        }
    }
}

impl Drop for EchoGuard {
    fn drop(&mut self) {
        if self.suppressed {
            set_echo(true);
        }
    }
}

fn set_echo(on: bool) -> bool {
    let flags: &[&str] = if on {
        &["echo", "echonl"]
    } else {
        &["-echo", "-echonl"]
    };

    let direct = std::process::Command::new("stty")
        .args(flags)
        .stdin(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::null())
        .status();
    if let Ok(status) = direct {
        if status.success() {
            return true;
        }
    }

    let script = format!("stty {} < /dev/tty", flags.join(" "));
    std::process::Command::new("sh")
        .arg("-c")
        .arg(&script)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

pub fn restore_echo() {
    set_echo(true);
}

pub fn newline() {
    println!();
    let _ = std::io::Write::flush(&mut std::io::stdout());
}

#[allow(dead_code)]
pub fn gap() {
    LINES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    println!();
    let _ = std::io::Write::flush(&mut std::io::stdout());
}

pub fn squeeze(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut sp = false;
    for c in input.chars() {
        if c.is_control() || c == '\u{feff}' {
            if c == '\n' || c == '\r' || c == '\t' {
                sp = true;
            }
            continue;
        }
        if c.is_whitespace() {
            sp = true;
            continue;
        }
        if sp && !out.is_empty() {
            out.push(' ');
        }
        sp = false;
        out.push(c);
    }
    out
}

pub fn sanitize(input: &str) -> String {
    let mut out = squeeze(input);
    for key in [
        "accessToken",
        "access_token",
        "refreshToken",
        "initData",
        "init_data",
        "Authorization",
        "idToken",
        "id_token",
        "password",
    ] {
        out = redact_after(&out, key);
    }
    out
}

pub fn reason(input: &str) -> &'static str {
    let text = sanitize(input).to_ascii_lowercase();
    if text.contains("timed out") || text.contains("timeout") {
        "the request timed out"
    } else if text.contains("connect") {
        "the connection could not be established"
    } else if text.contains("dns") || text.contains("resolve") || text.contains("name or service") {
        "the address could not be resolved"
    } else if text.contains("body") || text.contains("decode") || text.contains("json") {
        "the reply could not be read"
    } else if text.contains("config") || text.contains("data.txt") || text.contains("proxy.txt") {
        "the local files are not ready"
    } else {
        "the request failed"
    }
}

pub fn explain(e: &Error) -> String {
    match e {
        Error::Cfg(_) => "the local files are not ready".to_string(),
        Error::Auth(msg) | Error::Gate(msg) | Error::State(msg) => sanitize(msg),
        Error::Api { status, .. } => format!("the server answered {}", status),
        Error::Http(msg) => msg.clone(),
        other => reason(&other.to_string()).to_string(),
    }
}

fn redact_after(input: &str, key: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut rest = input;
    while let Some(pos) = rest.find(key) {
        out.push_str(&rest[..pos]);
        out.push_str(key);
        let tail = &rest[pos + key.len()..];
        let mut idx = 0;
        let bytes = tail.as_bytes();
        while idx < bytes.len()
            && (bytes[idx] == b'"'
                || bytes[idx] == b':'
                || bytes[idx] == b'='
                || bytes[idx] == b' ')
        {
            out.push(bytes[idx] as char);
            idx += 1;
        }
        while idx < bytes.len() {
            let c = bytes[idx] as char;
            if c == '"' || c == '&' || c == ' ' || c == ',' || c == '}' || c == '\n' {
                break;
            }
            idx += 1;
        }
        out.push_str("***");
        rest = &tail[idx..];
    }
    out.push_str(rest);
    out
}

pub fn money(value: &str) -> String {
    format!("{}{}{}", G, value, Z)
}

pub fn mask_host(host: &str) -> String {
    let parts: Vec<&str> = host.split('.').collect();
    if parts.len() == 4 {
        return format!("{}*****{}", parts[0], parts[3]);
    }
    let chars: Vec<char> = host.chars().collect();
    if chars.len() > 4 {
        let head: String = chars.iter().take(2).collect();
        let tail: String = chars
            .iter()
            .rev()
            .take(2)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        format!("{}*****{}", head, tail)
    } else {
        "***".to_string()
    }
}

pub fn mask_proxy(line: &str) -> String {
    let mut rest = line.trim();
    if let Some(pos) = rest.find("://") {
        rest = &rest[pos + 3..];
    }
    if let Some(at) = rest.rfind('@') {
        rest = &rest[at + 1..];
    }
    let (host, port) = match rest.rsplit_once(':') {
        Some((host, port)) if port.chars().all(|c| c.is_ascii_digit()) && !port.is_empty() => {
            (host, port)
        }
        _ => (rest, ""),
    };
    let shown = mask_host(host);
    if port.is_empty() {
        format!("http://user:pass@{}", shown)
    } else {
        format!("http://user:pass@{}:{}", shown, port)
    }
}

pub fn mask_mail(mail: &str) -> String {
    let mut parts = mail.splitn(2, '@');
    let user = parts.next().unwrap_or("");
    let domain = parts.next().unwrap_or("");
    let head: String = user.chars().take(3).collect();
    if domain.is_empty() {
        return format!("{}*****", head);
    }
    format!("{}*****@{}", head, domain)
}

pub fn mask_phone(phone: &str) -> String {
    let digits: String = phone.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() < 7 {
        return "*".repeat(10);
    }
    let head: String = digits.chars().take(4).collect();
    let tail: String = digits
        .chars()
        .rev()
        .take(3)
        .collect::<Vec<char>>()
        .into_iter()
        .rev()
        .collect();
    format!("{}*****{}", head, tail)
}

fn short(input: &str, max: usize) -> String {
    let s = squeeze(input);
    if s.chars().count() <= max {
        return s;
    }
    let cut: String = s.chars().take(max + 1).collect();
    match cut.rfind(' ') {
        Some(i) if i > 0 => cut[..i].trim_end().to_string(),
        _ => s.chars().take(max).collect(),
    }
}

pub fn clamp(input: &str) -> String {
    short(input, LINE_MAX)
}

pub fn unit_word<'a>(value: u64, singular: &'a str, plural: &'a str) -> &'a str {
    if value == 1 {
        singular
    } else {
        plural
    }
}

fn left_text(left: u64) -> String {
    let days = left / 86400;
    let hours = (left % 86400) / 3600;
    let minutes = (left % 3600) / 60;
    let seconds = left % 60;
    if days > 0 {
        return format!("{:02}:{:02}:{:02}:{:02}", days, hours, minutes, seconds);
    }
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn countdown(seconds: u64, label: &str) {
    let text = squeeze(label);
    let mut left = seconds;
    LIVE.store(true, std::sync::atomic::Ordering::Relaxed);
    while left > 0 {
        print!("\r{}{} {}{}", Y, text, left_text(left), Z);
        let _ = std::io::Write::flush(&mut std::io::stdout());
        std::thread::sleep(std::time::Duration::from_secs(1));
        left -= 1;
    }
    blank();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn left_text_carries_days() {
        assert_eq!(left_text(45), "00:00:45");
        assert_eq!(left_text(3661), "01:01:01");
        assert_eq!(
            left_text(2 * 86400 + 3 * 3600 + 47 * 60 + 33),
            "02:03:47:33"
        );
    }

    #[test]
    fn a_prompt_keeps_the_colon_and_one_space() {
        assert_eq!(
            format!("{} ", squeeze("Enter the sign in link:")),
            "Enter the sign in link: "
        );
    }

    #[test]
    fn unit_word_pluralises() {
        assert_eq!(unit_word(1, "service is", "services are"), "service is");
        assert_eq!(unit_word(4, "service is", "services are"), "services are");
    }

    #[test]
    fn reason_never_carries_a_link() {
        let raw =
            "error sending request for uri (https://example.com/aGVsbG8): operation timed out";
        let text = reason(raw);
        assert_eq!(text, "the request timed out");
        assert!(!text.contains("http"), "link survived: {}", text);
        assert!(!text.contains("example"), "host survived: {}", text);
        assert_eq!(
            reason("dns error: failed to lookup address"),
            "the address could not be resolved"
        );
        assert_eq!(
            reason("error decoding response body"),
            "the reply could not be read"
        );
        assert_eq!(
            reason("connection refused"),
            "the connection could not be established"
        );
        assert_eq!(reason("something odd happened"), "the request failed");
    }

    #[test]
    fn clamp_keeps_a_long_line_at_the_limit() {
        let long = "word ".repeat(80);
        assert!(clamp(&long).chars().count() <= LINE_MAX);
    }

    #[test]
    fn mask_keeps_the_domain_and_hides_the_user() {
        assert_eq!(mask_mail("someone@gmail.com"), "som*****@gmail.com");
    }

    #[test]
    fn masked_proxy_hides_the_middle_and_the_password() {
        let shown = mask_proxy("socks5://user:secret@74.81.20.100:10043");
        assert_eq!(shown, "http://user:pass@74*****100:10043");
        assert!(!shown.contains("secret"), "password survived: {}", shown);
        assert_eq!(
            mask_proxy("http://user:secret@185.20.30.81:8000"),
            "http://user:pass@185*****81:8000"
        );
    }
}
