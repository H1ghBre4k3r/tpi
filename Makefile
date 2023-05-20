.DEFAULT_GOAL := build

ifdef release 
	RELEASE=--release
endif

build:
	cargo build --bin tpi $(RELEASE)
