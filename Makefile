.PHONY: help build release clean \
	fmt fmt-check clippy test tests check audit deny machete changelog

.DEFAULT_GOAL := help

help:
	@echo "Available targets:"
	@echo "  make rust-build      - build the Rust workspace in debug mode"
	@echo "  make release         - build the Rust workspace in release mode"
	@echo "  make clean           - remove build/ and cargo outputs"
	@echo "  make check           - run formatting, lint and Rust tests"
	@echo "  make changelog       - regenerate CHANGELOG.md with git-cliff"


fmt:
	@cargo fmt --all

fmt-check:
	@cargo fmt --all -- --check

clippy:
	@cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	@cargo test --workspace --locked

tests: test

audit:
	@cargo audit

deny:
	@cargo deny check

machete:
	@cargo machete

check: fmt-check clippy test

build:
	@cargo build --workspace --locked

release: check
	@cargo build --workspace --release --locked

changelog:
	@git-cliff -o CHANGELOG.md

clean:
	@cargo clean
