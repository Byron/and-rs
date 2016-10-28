`and` [![Build Status](https://travis-ci.org/Byron/and-rs.svg?branch=master)](https://travis-ci.org/Byron/and-rs) (pronounced `anders`) is a tool to help with the typical android project development process, and essentially is a convenience wrapper around the lower level plumbing provided by the Android-SDK.

#### How to ...

* Learn about what the build system can do
 + `make`
* Run acceptance tests
 + `make tests`
* Build the release-binary for the current platform
 + `make release-build`

*Please note:* that make will install `rust` as project-local asset, provided you have no system installation. In the latter case, it will try to assure it is up-to-date by running `rustup|multirust update stable` just once.

#### Project Goals
* Fulfill uber technical assignment acceptance criteria
* run on all platforms as the Android-SDK
* be sufficiently easy to maintain and extend

#### Project Non-Goals
* be constrained in any way
* Be easy to build on plain Windows due to the use of standard linux tools like `make` and `bash`.

#### About the choice of tools
* `make`
 + The project hub. It's the go-to tool to keep track of dependencies between other tools and files to aid the developer.
* `bash`
 + Makes it easy to orchestrate processes and to automate the project. Very efficient in conjunction with make.
* `Rust`
 + Safe programming language that makes it easy to seed most common bugs out at compile-time. Makes it easy to write high-quality, highly maintainable software.
 + Also I want to see if one can optimize the code written for developer productivity by keeping things very straightforward.
 + Otherwise I would have loved to use `crystal`, which to me is like a more simple Rust. However, it doesn't run on Windows yet.
* `Ruby`
 + RSpecs are good-looking and easy to write, but maybe I am just a Ruby fan.
 + Even more a of a fan I am towards `crystal`, but it doesn't run on Windows just yet. If the latter is not a requirement, I would totally use it though as it
   appears to me as a better (statically type-checked, faster, but no less ergonomic) ruby.
 
#### Limitations
 * This project was never tested on Windows, but _should_ work if run in an environment supporting `make` and `bash`.
 * Only `init-osx` is provided to help developer to get started. Other platforms are out of scope. It would be possible to install them only locally, using the [installation steps provided here][manual-android-platform-tools]
 * During ruby installation, you will be asked for your password without knowing what it is for. Ideally, one would swap-in a `sudo` that provides more information, and allows to skip executing the installation command entirely.
 * If you use `rvm`, `rspec` might simply not work. I uninstalled it, never liked its intrusiveness, and given how easy a local installation is, that is the way to go for me from now on. Lesson learned.
 * The makefile is fine trying to use the system-provided ruby installation if present but does not check the version. It could be possible that older ruby versions are not compatible with the syntax used in the specs. As the CI runs Ruby 1.9, I will keep my code compatible to that and we should be fine.
   
[manual-android-platform-tools]: http://stackoverflow.com/questions/31374085/installing-adb-on-mac-os-x