.PHONY: build build-dev
build: build-dev

build-dev:
	cargo build

build-release:
	cargo build --release

clean:
	cargo clean
