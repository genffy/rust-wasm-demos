TAG ?= latest
PLATFORM ?= linux/amd64,linux/arm64
VERSION ?= latest

CARGO_TARGET_DIR ?= $(CURDIR)/target

lint:
	cargo fmt --all
	cargo clippy --workspace --all-targets -- -D warnings

build:
	wasm-pack build --target web

test:
# TODO check chromedriver is exists.
	wasm-pack test --chrome

clean:
	cargo clean

web:
	python3 -m http.server 8080 --bind 0.0.0.0 -d ./

pack:
	wasm-pack pack

# publish:
# 	npm config set registry https://registry.npmjs.org
# 	npm login
# 	wasm-pack publish
