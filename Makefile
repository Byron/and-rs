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
RUBY_IN_ENVIRONMENT := $(shell command -v ruby 2>&1)
RUBY=$(abspath $(RUBY_INSTALLDIR)/bin/ruby)
RSPEC=$(abspath $(RUBY_INSTALLDIR)/bin/rspec)
AND_EXECUTABLE_DEBUG=src/cli/target/debug/and
AND_EXECUTABLE_RELEASE=src/cli/target/release/and
RUST_SOURCE_FILES=$(shell find src -name '*.rs' -type f)

ifeq ($(RUBY_IN_ENVIRONMENT),)
$(RUBY):
	@-rm -Rf .tmp
	curl -sSLo ri.tar.gz https://github.com/postmodern/ruby-install/archive/v0.6.0.tar.gz
	mkdir -p .tmp && tar --strip 1 -xzvf ri.tar.gz -C .tmp && rm ri.tar.gz
	.tmp/bin/ruby-install -j2 --install-dir $(abspath $(RUBY_INSTALLDIR)) ruby 2.3 -- --disable-install-rdoc
	rm -Rf .tmp
else
$(RUBY):
	@echo Using system ruby installation
	mkdir -p $(dir $@) && ln -s $(RUBY_IN_ENVIRONMENT) $@
endif

$(RSPEC): $(RUBY)
	$(dir $<)/gem install rspec

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
	@bin/check.sh
	
tests: $(AND_EXECUTABLE_DEBUG) check $(RSPEC)
	EXECUTABLE=$(AND_EXECUTABLE_DEBUG) $(RSPEC) --fail-fast --format documentation --color tests/*.rb
	
$(DIST_DIR)/and: $(AND_EXECUTABLE_RELEASE)
	@mkdir -p $(DIST_DIR)
	@cp $< $@
	@echo "Release build ready at $@"
	
release-build: $(DIST_DIR)/and
	
init-osx:
	brew install android-sdk
	
clean:
	rm -Rf $(RUST_INSTALLDIR)
	rm -Rf $(RUBY_INSTALLDIR)
	rm -Rf $(DIST_DIR)
	cd src/cli && cargo clean
	cd src/lib && cargo clean
