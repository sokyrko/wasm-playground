.PHONY: wasmer-simple wasmer-json-exchange

wasmer-simple:
	cd ./wasmer-simple && cargo run

wasmer-json-exchange:
	cd ./wasmer-json-exchange/guest && \
	cargo build --release --target wasm32-unknown-unknown && \
	cargo run -p host
