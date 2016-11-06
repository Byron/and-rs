.PHONY: help clean package launch

help:
	$(info Make targets)
	$(info ------------)
	$(info package       | create a signed package of all you code and resources)
	$(info launch        | send the signed package to the running emulator)
	$(info clean         | remove all directories actively managed by this makefile)
	$(info)

RUST_INSTALLDIR=.rust
DIST_DIR=bin
TMP_DIR=.tmp
CARGO=$(abspath $(RUST_INSTALLDIR)/bin/cargo)
ANDERS_WORKTREE=$(TMP_DIR)/and-rs
ANDERS_EXECUTABLE_RELEASE=$(ANDERS_WORKTREE)/build/release/anders
ANDERS=$(DIST_DIR)/anders
JAVA_SOURCE_FILES=$(shell find src -name '*.java' -type f)
LIBRARY_FILES=$(shell find lib -name '*.jar' -type f)
RESOURCE_FILES=$(shell find obj -name '*.xml' -type f)
SIGNED_PACKAGE=bin/${project}.apk

$(CARGO):
	RUSTUP_HOME="$(RUST_INSTALLDIR)" CARGO_HOME="$(RUST_INSTALLDIR)" bash -c 'curl https://sh.rustup.rs -sSf | sh -s -- -y'
	$(RUST_INSTALLDIR)/bin/rustup default stable
	
$(TMP_DIR):
	mkdir -p $@

$(ANDERS_WORKTREE): $(TMP_DIR)
	git clone https://github.com/Byron/and-rs $@
	
$(ANDERS_EXECUTABLE_RELEASE): $(ANDERS_WORKTREE) $(CARGO)
	cd $(ANDERS_WORKTREE)/src/cli && $(CARGO) build --release
	strip $@
	
test: $(CARGO)
	cd src/lib && $(CARGO) test
	
$(ANDERS): $(ANDERS_EXECUTABLE_RELEASE)
	@mkdir -p $(DIST_DIR)
	@cp $< $@
	@$(MAKE) clean
	@echo "Release build ready at $@"
	
$(SIGNED_PACKAGE): $(ANDERS) $(JAVA_SOURCE_FILES) $(LIBRARY_FILES) $(RESOURCE_FILES)
	$(ANDERS) compile
	$(ANDERS) package
	
package: $(SIGNED_PACKAGE)
	
launch: $(SIGNED_PACKAGE)
	$(ANDERS) launch
	
clean:
	rm -Rf $(RUST_INSTALLDIR)
	rm -Rf $(TMP_DIR)
