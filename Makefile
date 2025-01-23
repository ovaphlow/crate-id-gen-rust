.PHONY: all test build build-windows

all: build

test:
	cargo test -- --nocapture

build:
	cargo build --release

build-windows:
	rustup target add x86_64-pc-windows-gnu
	cargo build --target x86_64-pc-windows-gnu --release
