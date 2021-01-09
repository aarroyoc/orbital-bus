.PHONY: all
all:
	cargo build --target wasm32-unknown-unknown
	wasm-bindgen --out-dir public --target web target/wasm32-unknown-unknown/debug/orbital_bus.wasm

.PHONY: server
server:
	cd public && python3 -m http.server