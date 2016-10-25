.PHONY: help clean tests init-osx

help:
	$(info Make targets)
	$(info ------------)
	$(info tests    | run `and` against a suite of tests to assure it works)
	$(info init-osx | WARNING: affects system: install android tools needed for basic android work)
	$(info)

include .make-config.env
CARGO_IN_ENVIRONMENT := $(shell command -v cargo 2>&1)
CARGO=$(RUST_INSTALLDIR)/bin/cargo
AND_EXECUTABLE_DEBUG=target/debug/and
RUST_SOURCE_FILES=$(wildcard **/*.rs)

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

$(AND_EXECUTABLE_DEBUG): $(RUST_SOURCE_FILES) $(CARGO)
	$(CARGO) build
	
tests: $(AND_EXECUTABLE_DEBUG)
	bin/tests.sh $(AND_EXECUTABLE_DEBUG)
	
init-osx:
	brew install android-sdk
	
clean:
	rm -Rf $(RUST_INSTALLDIR)
