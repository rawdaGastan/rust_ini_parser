verifiers: fmt check clippy

fmt:
	cargo fmt

check:
	cargo check

clippy:
	cargo clippy

test: verifiers
	cargo test

build: 
	cargo build

coverage:  
	cargo install cargo-tarpaulin
	cargo tarpaulin

