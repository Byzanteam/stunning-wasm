compile:
	cargo build --target wasm32-unknown-unknown --release

	cd slp_api/flows && cargo build --target wasm32-unknown-unknown --release

.PHONY: compile
