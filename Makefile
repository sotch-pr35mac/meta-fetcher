check:
	cargo check && \
	cargo clippy

check-ci:
	cargo check && \
	cargo fmt --check && \
	cargo clippy

test:
	cargo test --features network-tests

test-ci:
	cargo test

