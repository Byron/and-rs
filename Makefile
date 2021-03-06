.PHONY: help clean spec test init-osx release purge

help:
	$(info Make targets)
	$(info ------------)
	$(info spec          | run `anders` against a suite of specifications to assure it works)
	$(info test          | run unit tests of all rust code)
	$(info release       | build a release binary of the anders tool)
	$(info init-osx      | WARNING: affects system: install android tools needed for basic android work)
	$(info clean         | remove all directories actively managed by this makefile)
	$(info purge         | reset this folder back to a pristine state using git clean -fxd)
	$(info)

RUST_INSTALLDIR=.rust
CRYSTAL_INSTALLDIR=.crystal
DIST_DIR=dist
CARGO=$(abspath $(RUST_INSTALLDIR)/bin/cargo)
CRYSTAL=$(abspath $(CRYSTAL_INSTALLDIR)/bin/crystal)
ANDERS_EXECUTABLE_DEBUG=build/debug/anders
ANDERS_EXECUTABLE_RELEASE=build/release/anders
RUST_SOURCE_FILES=$(shell find src -name '*.rs' -or -name '*.cr' -type f)
CARGO_TOML_FILES=$(shell find src -name 'Cargo.toml' -type f)
CRYSTAL_SOURCE_FILES=$(shell find spec -name '*.cr' -type f)
SPEC_EXECUTABLE=.tmp/spec-runner
SPEC_OK=spec/.ok

$(CRYSTAL):
	@bin/check.sh basic
	curl -sSLo cr.tar.gz https://github.com/crystal-lang/crystal/releases/download/0.19.4/crystal-0.19.4-1-`uname -s | tr '[:upper:]' '[:lower:]'`-`uname -m`.tar.gz
	mkdir -p $(CRYSTAL_INSTALLDIR) && tar --strip 1 -xzf cr.tar.gz -C $(CRYSTAL_INSTALLDIR) && rm cr.tar.gz

$(CARGO):
	RUSTUP_HOME="$(RUST_INSTALLDIR)" CARGO_HOME="$(RUST_INSTALLDIR)" bash -c 'curl https://sh.rustup.rs -sSf | sh -s -- -y'
	$(RUST_INSTALLDIR)/bin/rustup default stable

$(ANDERS_EXECUTABLE_RELEASE): $(RUST_SOURCE_FILES) $(CARGO) $(CARGO_TOML_FILES)
	cd src/cli && $(CARGO) build --release
	strip $@
	
$(ANDERS_EXECUTABLE_DEBUG): $(RUST_SOURCE_FILES) $(CARGO) $(CARGO_TOML_FILES)
	cd src/cli && $(CARGO) build
	
libs/zip-crystal: $(CRYSTAL)
	$(CRYSTAL) deps
	
$(SPEC_EXECUTABLE): $(CRYSTAL) $(CRYSTAL_SOURCE_FILES) libs/zip-crystal
	@mkdir -p $(dir $@)
	$(CRYSTAL) build -o $@ spec/acceptance_spec.cr
	
$(SPEC_OK): $(ANDERS_EXECUTABLE_DEBUG) $(SPEC_EXECUTABLE)
	@bin/check.sh all
	EXECUTABLE=$(abspath $(ANDERS_EXECUTABLE_DEBUG)) $(SPEC_EXECUTABLE) && touch $(SPEC_OK) || { rm -f $(SPEC_OK) && exit 3; }
	
spec: $(SPEC_OK)
	
test: $(CARGO)
	cd src/lib && $(CARGO) test
	
$(DIST_DIR)/anders: $(ANDERS_EXECUTABLE_RELEASE)
	@mkdir -p $(DIST_DIR)
	@cp $< $@
	@echo "Release build ready at $@"
	
release: $(DIST_DIR)/anders
	
init-osx:
	brew install android-sdk
	
purge:
	git clean -fxd
clean:
	rm -Rf $(RUST_INSTALLDIR)
	rm -Rf $(CRYSTAL_INSTALLDIR)
	rm -Rf $(DIST_DIR)
	rm -Rf .tmp
	rm -f $(SPEC_OK)
	cd src/cli && cargo clean
	cd src/lib && cargo clean
