.PHONY: help

compile:
	cargo build

lint:
	cargo clippy --fix