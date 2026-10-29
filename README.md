<div align="center">

<img width="100%" alt="header" src="https://capsule-render.vercel.app/api?type=waving&height=210&text=Attapoll%20Bot&fontAlign=50&fontAlignY=36&fontSize=56&desc=Survey%20Panel%20%7C%20Projects%20%7C%20Streak%20%7C%20Offerwall%20%7C%20Cashout&descAlign=50&descAlignY=58"/>

<img alt="typing" src="https://readme-typing-svg.demolab.com?font=Inter&size=18&duration=3000&pause=650&center=true&vCenter=true&width=900&lines=Magic+Link+Auth+%7C+No+Password+Required;Auto+Projects+%7C+Survey+Activity+%26+Eligibility;Auto+Daily+Streak+%7C+Maintain+Check-in;Auto+Offerwall+%7C+AdGem+%26+Adjoe+Credit;Auto+Cashout+%7C+PayPal+%2F+Gift+Card;Proxy+Support+%7C+Multi-Account"/>

<p>
  <img alt="rust" src="https://img.shields.io/badge/Rust-2021-f74c00?logo=rust&logoColor=white"/>
  <img alt="platform" src="https://img.shields.io/badge/Platform-AttaPoll-111111"/>
  <img alt="multi-account" src="https://img.shields.io/badge/Multi--Account-Supported-111111"/>
  <img alt="author" src="https://img.shields.io/badge/by-Yuurisandesu-111111"/>
</p>

<p>
  <b>Attapoll Bot</b> is an automation client for the AttaPoll survey panel.<br/>
  It signs in via email magic link with no password, caches the session, and on every cycle processes available survey projects, maintains the daily streak, credits offerwall rewards from AdGem and Adjoe, and optionally requests a cashout to PayPal or a gift card, all across multiple accounts with proxy support and a live countdown between cycles.<br/>
  Built and distributed by <b>Yuurisandesu</b>.
</p>

</div>

---

## Table of Contents

- [Requirements](#requirements)
- [Install Rust](#install-rust)
- [Installation](#installation)
- [Configuration](#configuration)
- [First-Time Sign In](#first-time-sign-in)
- [Running the Bot](#running-the-bot)
- [Run Modes](#run-modes)
- [Features](#features)
- [File Structure](#file-structure)
- [Disclaimer](#disclaimer)

---

## Requirements

- Rust `1.80+` (includes `cargo`, only needed if building from source)
- Git

---

## Install Rust

### Linux / macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify:

```bash
rustc --version
```

### Windows

Download and run the installer from https://rustup.rs, then reopen the terminal and verify:

```cmd
rustc --version
```

### Termux (Android)

```bash
pkg update && pkg install proot-distro
proot-distro install ubuntu
proot-distro login ubuntu
```

Then inside Ubuntu:

```bash
apt update && apt install -y curl git build-essential pkg-config libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

---

## Installation

**Clone the repository:**

```bash
git clone https://github.com/Yuurisan-N1/Attapoll-Application.git
cd Attapoll-Application
```

**Build the binary:**

```bash
cargo build --release
# or
./run.sh build
```

---

## Configuration

### 1. Accounts (data.txt)

Fill `data.txt` with one email address per line. There is no password. Blank lines and lines starting with `#` are ignored:

```
you@example.com
another@example.com
```

### 2. Proxy (proxy.txt)

Fill `proxy.txt` with proxies, one per line (optional, leave empty to run without proxy):

```
http://user:pass@host:port
socks5://user:pass@host:port
```

Proxies are paired to accounts by index.

### 3. Bot Settings (config.json)

| Setting | Description |
|---|---|
| `settings.sleep_seconds` | Seconds to wait between cycles |
| `settings.retries` | Number of retries on transient network errors |
| `settings.gates` | Set to `true` to attempt device integrity gates. Leave `false` to skip them |
| `payout.method` | Cashout method: `PAYPAL` or a gift card provider name |
| `payout.account` | PayPal email or gift card account to receive the payout |

---

## First-Time Sign In

AttaPoll uses email magic links. There is no password. The first run for each account sends a sign-in link to the inbox and waits for you to paste it back.

**Option A - Interactive (the bot asks for the link):**

```bash
./run.sh link you@example.com
```

The bot sends the link to your inbox and waits. Paste the full URL from the email when prompted.

**Option B - One command:**

```bash
./run.sh signin-link you@example.com "https://the-full-link-from-the-email"
```

After a successful sign-in, the session is saved to `tokens.json`. Subsequent runs load from the cache and no link is needed again. If the session expires, run the link step again.

**Verify the sign-in worked:**

```bash
./run.sh probe
```

---

## Running the Bot

Make `run.sh` executable first:

```bash
chmod +x run.sh
```

Then choose a mode:

```bash
./run.sh run          # build if needed and run the endless loop in foreground
./run.sh once         # build if needed and run a single cycle then exit
./run.sh bg           # run the endless loop in background, logs saved to attapoll-bot.log
./run.sh bgprobe      # probe in background
./run.sh screen       # run the endless loop in a detached screen session
./run.sh stop         # stop background or screen session
./run.sh probe        # read profile and project list without claiming anything
./run.sh selftest     # offline checks, no network
./run.sh build        # build only, do not run
```

For background mode, tail the log anytime:

```bash
tail -f attapoll-bot.log
```

For screen mode, attach anytime:

```bash
screen -r attapoll-bot
```

Press `Ctrl+C` once to stop the bot cleanly when running in foreground.

### Using make (Linux / macOS)

```bash
make release    # optimized release build
make start      # build release and run immediately
make run        # build debug and run
make check      # check for errors without building
make fmt        # format the code
make clean      # remove all build artifacts
make size       # show release binary size
```

### Manual run

```bash
# Linux / macOS / Termux
./target/release/attapoll-bot run

# Windows
.\target\release\attapoll-bot.exe run
```

---

## Run Modes

| Mode | Command | Description |
|---|---|---|
| run | `./run.sh run` | Endless loop: projects, streak, offerwall, cashout, sleep |
| once | `./run.sh once` | Single cycle then exit |
| projects | `./run.sh projects` | Run only the projects step |
| streak | `./run.sh streak` | Run only the streak step |
| probe | `./run.sh probe` | Read profile and project list without claiming |
| selftest | `./run.sh selftest` | Offline checks, no network |
| verify | `./run.sh verify` | Re-verify the session for all accounts |
| cashout | `./run.sh cashout <amount>` | Request a cashout for the given amount |
| referral | `./run.sh referral <code>` | Apply a referral code |
| hide | `./run.sh hide <offer>` | Hide a specific offerwall offer |
| link | `./run.sh link <mail>` | Send the sign-in link to the inbox and wait for it |
| signin-link | `./run.sh signin-link <mail> <link>` | Submit the full magic link directly |
| bg | `./run.sh bg` | Run endless loop in background with log file |
| bgprobe | `./run.sh bgprobe` | Probe in background |
| screen | `./run.sh screen` | Run endless loop in a detached screen session |
| stop | `./run.sh stop` | Stop background or screen session |
| build | `./run.sh build` | Build only |

---

## Features

### Magic Link Authentication
Each account signs in with an email magic link. No password is stored or required. The session token is saved to `tokens.json` after the first sign-in and reused on all subsequent runs. When the session expires, a new link is requested with `./run.sh link`.

### Auto Projects
The bot fetches the list of available survey projects, checks eligibility and activity status, and submits project responses. Project history is tracked to avoid re-attempting completed ones. Gate checks (phone or device integrity) are logged and skipped without stopping the run.

### Daily Streak
The bot checks streak progress and terms for each account and submits the streak check-in when due. If the streak is already claimed or terms are not met, it is skipped with a note.

### Offerwall Credit
AdGem and Adjoe offerwall credits are fetched and applied for each account. Offers that have already been credited are skipped automatically.

### Auto Cashout
When a payout method and account are configured, the bot requests a cashout for the available balance. The available counterparty options are fetched from the server before each cashout. Gift card redemption is also supported alongside PayPal.

### Referral
Referral codes can be applied with `./run.sh referral <code>`. Referral bonuses are fetched and logged per cycle.

### Multi Account
All accounts in `data.txt` are processed sequentially within every cycle. Each account uses its own session from `tokens.json` and its own proxy from the pool.

### Proxy Support
Proxies are loaded from `proxy.txt` and paired to accounts by index. HTTP and SOCKS5 are both supported. Proxy credentials are masked in log output. Running without proxies is fully supported.

### Auto Countdown
After all accounts complete a cycle, the bot displays a live `HH:MM:SS` countdown until the next cycle starts.

---

## File Structure

```text
Attapoll-Application/
├── .github/
│   └── workflows/
│       └── build.yml                    # CI: build and publish to Releases on version tag
├── src/
│   ├── main.rs                          # Entry point, Ctrl+C handler, mode dispatch
│   ├── auth/
│   │   ├── email.rs                     # Magic link request
│   │   ├── link.rs                      # Magic link submission
│   │   ├── session.rs                   # Session cache: tokens.json read and write
│   │   └── mod.rs
│   ├── cashout/
│   │   ├── counterparty.rs              # Counterparty options fetch
│   │   ├── giftcard.rs                  # Gift card redemption
│   │   ├── options.rs                   # Cashout option list
│   │   ├── request.rs                   # Cashout request submit
│   │   └── mod.rs
│   ├── cli/
│   │   ├── dispatch.rs                  # Mode selector and argument parser
│   │   ├── probe.rs                     # Probe mode: read without claiming
│   │   ├── selftest.rs                  # Offline checks
│   │   ├── verify.rs                    # Session verify
│   │   └── mod.rs
│   ├── integrity/
│   │   ├── access.rs                    # Device access gate
│   │   ├── install.rs                   # Install integrity check
│   │   ├── play.rs                      # Play integrity gate
│   │   ├── verify.rs                    # Integrity verify
│   │   └── mod.rs
│   ├── net/
│   │   ├── client.rs                    # HTTP client with cookie jar
│   │   ├── egress.rs                    # Proxy pool loader and masker
│   │   ├── headers.rs                   # Request headers builder
│   │   └── mod.rs
│   ├── offerwall/
│   │   ├── adgem.rs                     # AdGem credit fetch and apply
│   │   ├── adjoe.rs                     # Adjoe credit fetch and apply
│   │   ├── credit.rs                    # Credit submission
│   │   └── mod.rs
│   ├── projects/
│   │   ├── activity.rs                  # Project activity check
│   │   ├── history.rs                   # Completed project history
│   │   ├── list.rs                      # Project list fetch
│   │   ├── response.rs                  # Project response submit
│   │   ├── survey.rs                    # Survey session handler
│   │   ├── tests.rs                     # Project unit tests
│   │   └── mod.rs
│   ├── referral/
│   │   ├── apply.rs                     # Referral code apply
│   │   ├── bonus.rs                     # Referral bonus fetch
│   │   └── mod.rs
│   ├── run/
│   │   ├── account.rs                   # Per-account runner
│   │   ├── cycle.rs                     # Full cycle: projects, streak, offerwall, cashout
│   │   └── mod.rs
│   ├── streak/
│   │   ├── progress.rs                  # Streak progress fetch
│   │   ├── terms.rs                     # Streak terms check
│   │   └── mod.rs
│   └── support/
│       ├── banner.rs                    # ASCII banner
│       ├── inbox.rs                     # Email inbox helper
│       ├── cfg/                         # Config loader and schema
│       ├── consts/                      # App and network constants
│       ├── error.rs                     # Error types
│       ├── log.rs                       # Colored log output and countdown helper
│       └── mod.rs
├── assets/
│   └── icon.ico                         # Windows binary icon
├── Cargo.toml                           # Project manifest and dependencies
├── Cargo.lock
├── build.rs                             # Build script (embeds icon into Windows binary)
├── Makefile                             # make targets: build, run, release, start, clean, size
├── run.sh                               # Run helper: all modes
├── config.json                          # Sleep, retries, gates, payout settings
├── data.txt                             # Email addresses, one per line
├── tokens.json                          # Session cache per account (auto-generated)
├── HOWTO.txt                            # Quick start guide
└── LICENSE                              # GNU AGPL v3.0
```

---

## Disclaimer

This tool is built for educational and technical exploration purposes. Use it wisely and at your own responsibility.

---

<div align="center">
<img width="100%" alt="footer" src="https://capsule-render.vercel.app/api?type=waving&height=120&section=footer"/>
</div>