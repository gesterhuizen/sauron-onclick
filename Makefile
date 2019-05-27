build:
	wasm-pack build --target no-modules --dev -- --features "console_error_panic_hook"

server:
	basic-http-server ./ -a 0.0.0.0:4001

clean:
	cargo clean
	rm -fr pkg
