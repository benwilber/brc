build:
	cargo build

buildrelease:
	cargo build --release

format:
	cargo fmt

lint:
	cargo fmt --check
	cargo clippy