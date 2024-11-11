BINARY := display_switch
INTEL_ARCH := x86_64-apple-darwin
ARM_ARCH := aarch64-apple-darwin
UNAME_S := $(shell uname -s)
VERSION := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')

# Targets for different build modes
.PHONY: build-debug build-release all test clean setup-$(INTEL_ARCH) setup-$(ARM_ARCH)

# macOS specific debug build: creates a universal debug binary
ifeq ($(UNAME_S), Darwin)
build-debug: target/debug/$(BINARY)

build-release: target/release/$(BINARY)

setup-$(INTEL_ARCH):
	rustup target add $(INTEL_ARCH)

setup-$(ARM_ARCH):
	rustup target add $(ARM_ARCH)

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

target/$(INTEL_ARCH)/debug/$(BINARY): setup-$(INTEL_ARCH)
	cargo build --target $(INTEL_ARCH)

target/$(ARM_ARCH)/debug/$(BINARY): setup-$(ARM_ARCH)
	cargo build --target $(ARM_ARCH)

target/$(INTEL_ARCH)/release/$(BINARY): setup-$(INTEL_ARCH)
	cargo build --target $(INTEL_ARCH) --release

target/$(ARM_ARCH)/release/$(BINARY): setup-$(ARM_ARCH)
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

package-release: build-release
	mkdir -p "target/package"
	zip -j target/package/$(BINARY)-v$(VERSION)-$(PLATFORM).zip target/release/$(BINARY)* README.md LICENSE
	cp README.md LICENSE target/release/$(BINARY) target/package
	cd target/package && zip -r $(BINARY)-v$(VERSION)-$(PLATFORM).zip $(BINARY) README.md LICENSE
	echo "$(BINARY)-v$(VERSION)-$(PLATFORM).zip"
