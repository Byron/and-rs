.PHONY: help clean tests init-osx release-build check

help:
	$(info Make targets)
	$(info ------------)
	$(info tests         | run `and` against a suite of tests to assure it works)
	$(info release-build | build a release binary of the and tool)
	$(info init-osx      | WARNING: affects system: install android tools needed for basic android work)
	$(info)

include .make-config.env
CARGO_IN_ENVIRONMENT := $(shell command -v cargo 2>&1)
CARGO=$(abspath $(RUST_INSTALLDIR)/bin/cargo)
AND_EXECUTABLE_DEBUG=src/cli/target/debug/and
AND_EXECUTABLE_RELEASE=src/cli/target/release/and
RUST_SOURCE_FILES=$(shell find src -name '*.rs' -type f)

ifeq ($(CARGO_IN_ENVIRONMENT),)
$(CARGO):
	RUSTUP_HOME="$(RUST_INSTALLDIR)" CARGO_HOME="$(RUST_INSTALLDIR)" bash -c 'curl https://sh.rustup.rs -sSf | sh -s -- -y'
	$(RUST_INSTALLDIR)/bin/rustup default stable
else
$(CARGO):
	@echo Using system rust installation and trying to assure it is uptodate
	-rustup update stable || multirust update stable
	mkdir -p $(dir $@) && ln -s $(CARGO_IN_ENVIRONMENT) $@
endif

$(AND_EXECUTABLE_RELEASE): $(RUST_SOURCE_FILES) $(CARGO)
	cd src/cli && $(CARGO) build --release
	
$(AND_EXECUTABLE_DEBUG): $(RUST_SOURCE_FILES) $(CARGO)
	cd src/cli && $(CARGO) build
	
check:
	bin/check.sh
	
tests: $(AND_EXECUTABLE_DEBUG) check
	bin/tests.sh $(AND_EXECUTABLE_DEBUG)
	
$(DIST_DIR)/and: $(AND_EXECUTABLE_RELEASE)
	@mkdir -p $(DIST_DIR)
	@cp $< $@
	@echo "Release build ready at $@"
	
release-build: $(DIST_DIR)/and
	
init-osx:
	brew install android-sdk
	
clean:
	rm -Rf $(RUST_INSTALLDIR)
	rm -Rf $(DIST_DIR)
	cd src/cli && cargo clean
	cd src/lib && cargo clean
