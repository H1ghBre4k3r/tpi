.DEFAULT_GOAL := build

ifdef release 
	RELEASE=--release
endif

build:
	cargo build --bin tpi $(RELEASE) && \
	cargo build --target armv7-unknown-linux-gnueabihf --bin tpi-server $(RELEASE)
