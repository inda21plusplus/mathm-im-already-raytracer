compile: src/
	cargo build --release --target wasm32-unknown-unknown

build: compile
	wasm-bindgen --out-dir www --target web ~/.cache/target/wasm32-unknown-unknown/release/web-demo.wasm --no-typescript
