compile:
	cargo build --target wasm32-unknown-unknown --release

	cd slp-api/flows && cargo build --target wasm32-unknown-unknown --release

.PHONY: compile
