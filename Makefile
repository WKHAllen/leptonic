all: build

build:
	cargo build

test:
	cargo test -- --nocapture

lint:
	cargo clippy -- -D warnings

demo:
	cd examples && trunk serve --open

clean:
	cargo clean
