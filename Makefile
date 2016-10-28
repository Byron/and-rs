.PHONY: help clean spec init-osx release-build

help:
	$(info Make targets)
	$(info ------------)
	$(info spec          | run `and` against a suite of specifications to assure it works)
	$(info release-build | build a release binary of the and tool)
	$(info init-osx      | WARNING: affects system: install android tools needed for basic android work)
	$(info)

include .make-config.env
CARGO_IN_ENVIRONMENT := $(shell command -v cargo 2>&1)
CARGO=$(abspath $(RUST_INSTALLDIR)/bin/cargo)
CRYSTAL=$(abspath $(CRYSTAL_INSTALLDIR)/bin/crystal)
AND_EXECUTABLE_DEBUG=src/cli/target/debug/and
AND_EXECUTABLE_RELEASE=src/cli/target/release/and
RUST_SOURCE_FILES=$(shell find src -name '*.rs' -type f)
CRYSTAL_SOURCE_FILES=$(shell find spec -name '*.cr' -type f)
SPEC_OK=spec/.ok

$(CRYSTAL):
	@bin/check.sh basic
	curl -sSLo cr.tar.gz https://github.com/crystal-lang/crystal/releases/download/0.19.4/crystal-0.19.4-1-`uname -s | tr '[:upper:]' '[:lower:]'`-`uname -m`.tar.gz
	mkdir -p $(CRYSTAL_INSTALLDIR) && tar --strip 1 -xzf cr.tar.gz -C $(CRYSTAL_INSTALLDIR) && rm cr.tar.gz

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
	
$(SPEC_OK): $(AND_EXECUTABLE_DEBUG) $(CRYSTAL) $(CRYSTAL_SOURCE_FILES)
	@bin/check.sh all
	EXECUTABLE=$(AND_EXECUTABLE_DEBUG) $(CRYSTAL) spec && touch $(SPEC_OK) || { rm -f $(SPEC_OK) && exit 3; }
	
spec: $(SPEC_OK)
	
$(DIST_DIR)/and: $(AND_EXECUTABLE_RELEASE)
	@mkdir -p $(DIST_DIR)
	@cp $< $@
	@echo "Release build ready at $@"
	
release-build: $(DIST_DIR)/and
	
init-osx:
	brew install android-sdk
	
clean:
	rm -Rf $(RUST_INSTALLDIR)
	rm -Rf $(CRYSTAL_INSTALLDIR)
	rm -Rf $(DIST_DIR)
	rm -f $(SPEC_OK)
	cd src/cli && cargo clean
	cd src/lib && cargo clean
