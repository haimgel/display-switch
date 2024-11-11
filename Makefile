BINARY := display_switch
INTEL_ARCH := x86_64-apple-darwin
ARM_ARCH := aarch64-apple-darwin
UNAME_S := $(shell uname -s)

# Targets for different build modes
.PHONY: build-debug build-release all test clean

# macOS specific debug build: creates a universal debug binary
ifeq ($(UNAME_S), Darwin)
build-debug: target/debug/$(BINARY)

build-release: target/release/$(BINARY)

target/debug/$(BINARY): target/$(INTEL_ARCH)/debug/$(BINARY) target/$(ARM_ARCH)/debug/$(BINARY)
	mkdir -p "target/debug"
	lipo -create -output $@ \
		"target/$(INTEL_ARCH)/debug/$(BINARY)" \
		"target/$(ARM_ARCH)/debug/$(BINARY)"

target/release/$(BINARY): target/$(INTEL_ARCH)/release/$(BINARY) target/$(ARM_ARCH)/release/$(BINARY)
	mkdir -p "target/release"
	lipo -create -output $@ \
		"target/$(INTEL_ARCH)/release/$(BINARY)" \
		"target/$(ARM_ARCH)/release/$(BINARY)"

target/$(INTEL_ARCH)/debug/$(BINARY):
	rustup target add $(INTEL_ARCH)
	cargo build --target $(INTEL_ARCH)

target/$(ARM_ARCH)/debug/$(BINARY):
	rustup target add $(ARM_ARCH)
	cargo build --target $(ARM_ARCH)

target/$(INTEL_ARCH)/release/$(BINARY):
	cargo build --target $(INTEL_ARCH) --release

target/$(ARM_ARCH)/release/$(BINARY):
	cargo build --target $(ARM_ARCH) --release

# Non-macOS build: defaults to standard cargo build
else
build-debug:
	cargo build

build-release:
	cargo build --release
endif

all: build-debug

test:
	cargo test

clean:
	cargo clean
