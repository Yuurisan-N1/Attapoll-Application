BINARY := attapoll-bot
RELEASE_DIR := target/release

.PHONY: all build run release start once check test lint fmt clean size

all: build

build:
	cargo build

run:
	cargo run

release:
	cargo build --release

start: release
	./$(RELEASE_DIR)/$(BINARY)

once: release
	./$(RELEASE_DIR)/$(BINARY) once

check:
	cargo check

test:
	cargo test

lint:
	cargo clippy --all-targets -- -D warnings

fmt:
	cargo fmt

clean:
	cargo clean

size: release
	@ls -lh $(RELEASE_DIR)/$(BINARY) | awk '{print "Binary size:", $$5}'
