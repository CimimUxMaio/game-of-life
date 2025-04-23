build:
	cargo build

run:
	cargo run

test:
	cargo test

clippy-check:
	cargo clippy -- -D warnings

format-check:
	cargo fmt --all -- --check

clean:
	cargo clean
