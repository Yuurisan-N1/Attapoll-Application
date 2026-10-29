#!/usr/bin/env bash
set -u

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT_DIR" || exit 1

BINARY="./target/release/attapoll-bot"
LOG_FILE="attapoll-bot.log"
SESSION_NAME="attapoll-bot"

build_release() {
    echo "Building release binary..."
    if ! cargo build --release; then
        echo "Build failed, aborting."
        exit 1
    fi
}

prepare() {
    if [ ! -f "data.txt" ]; then
        echo "data.txt not found, create it first (one mail address per line)."
        exit 1
    fi
    if [ ! -s "data.txt" ]; then
        echo "data.txt is empty, fill it with one mail address per line."
        exit 1
    fi
    if [ ! -x "$BINARY" ] || [ "Cargo.toml" -nt "$BINARY" ]; then
        build_release
    fi
}

mode_direct() {
    prepare
    exec "$BINARY" "${1:-run}"
}

mode_nohup() {
    prepare
    nohup "$BINARY" "${1:-run}" >> "$LOG_FILE" 2>&1 &
    echo "Started in background (PID $!)"
    echo "Logs: tail -f $LOG_FILE"
}

mode_screen() {
    if ! command -v screen >/dev/null 2>&1; then
        echo "screen not found, install with: apt install screen"
        exit 1
    fi
    prepare
    screen -dmS "$SESSION_NAME" "$BINARY" "${1:-run}"
    echo "Started in screen session '$SESSION_NAME'"
    echo "Attach : screen -r $SESSION_NAME"
    echo "Detach : Ctrl+A then D"
}

mode_stop() {
    if command -v screen >/dev/null 2>&1 && screen -ls | grep -q "$SESSION_NAME"; then
        screen -S "$SESSION_NAME" -X quit
        echo "Screen session stopped."
    fi
    if pkill -f "$BINARY" >/dev/null 2>&1; then
        echo "Process stopped."
    else
        echo "No running process found."
    fi
}

case "${1:-run}" in
    run|once|projects|streak|probe|selftest|verify)
        mode_direct "$1"
        ;;
    cashout|referral|hide)
        if [ -z "${2:-}" ]; then
            echo "Usage: ./run.sh $1 <value>"
            exit 1
        fi
        prepare
        exec "$BINARY" "$1" "$2"
        ;;
    link)
        if [ -z "${2:-}" ]; then
            echo "Usage: ./run.sh link <mail>"
            exit 1
        fi
        prepare
        exec "$BINARY" link "$2"
        ;;
    signin-link)
        if [ -z "${2:-}" ] || [ -z "${3:-}" ]; then
            echo "Usage: ./run.sh signin-link <mail> <link>"
            exit 1
        fi
        prepare
        exec "$BINARY" signin-link "$2" "$3"
        ;;
    bg)
        mode_nohup "run"
        ;;
    bgprobe)
        mode_nohup "probe"
        ;;
    screen)
        mode_screen "run"
        ;;
    stop)
        mode_stop
        ;;
    build)
        build_release
        ;;
    *)
        echo "Usage: ./run.sh [run|once|projects|streak|probe|selftest|verify|cashout <amount>|referral <code>|hide <offer>|link <mail>|signin-link <mail> <link>|bg|bgprobe|screen|stop|build]"
        exit 1
        ;;
esac
