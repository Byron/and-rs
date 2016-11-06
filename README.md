## `anders` is 'anders' [![Build Status](https://travis-ci.org/Byron/and-rs.svg?branch=master)](https://travis-ci.org/Byron/and-rs) ...
`anders` is a tool to help with the typical android project development process, and essentially is a convenience wrapper around the lower level plumbing provided by the Android-SDK.

Please note that in order to use it, you must have the Android SDK installed and the `ANDROID_HOME` environment variable set.

### How to use anders?

After installation via:
```
git clone https://github.com/Byron/and-rs && cd and-rs && make release
```

You will find the binary in `dist/anders`, and we will just assume it can be invoked with `anders` from now on.

Learn about what it can do with `anders -h`, and create a new project using:
```
anders new hello --package com.company.hello --target android-24
```

Change into the newly created directory and either go through the `compile`, `package` and `launch` sub-commands in order, or more easily run `make package` or `make launch`, which will launch `anders` for you after placing it into the _bin/_ directory.

If you want to put the project into `git` and send it to others, they will be able to do the same, just `make package` and they are ready to go.

### How to develop anders?

* Learn about what the build system can do
 + `make`
* Run acceptance tests
 + `make spec`
* Build the release-binary for the current platform
 + `make release`
* Initialize your system for android development. Please note that this operation will require root permissions. Also you will have to install the desired Android targets yourself, just run `android` afterwards.
 + `make init-osx`

### Everything Else

#### Project Goals
* fulfill uber technical assignment acceptance criteria
* run on all platforms as the Android-SDK
* be sufficiently easy to maintain and extend
* maximize quality, but give it some breathing room by choosing tests wisely

#### Project Non-Goals
* be constrained in any way
* be easy to build on plain Windows due to the use of standard linux tools like `make` and `bash`

#### About the choice of tools
* `make`
 + The project hub. It's the go-to tool to keep track of dependencies between other tools and files to aid the developer.
* `bash`
 + Makes it easy to orchestrate processes and to automate the project. Very efficient in conjunction with make.
* `Rust`
 + Safe programming language that makes it easy to seed most common bugs out at compile-time. Makes it easy to write high-quality, highly maintainable software.
 + Also I want to see if one can optimize the code written for developer productivity by keeping things very straightforward.
 + Otherwise I would have loved to use `crystal`, which to me is like a more simple Rust. However, it doesn't run on Windows yet.
* `Crystal`
 + specs are good-looking and easy to write, and that comes out of the box.
 + it's like a more ergonomic, easier to use and learn Rust. [Meet crystal][meet-crystal].
 + I consider it acceptable to not be able to run acceptance tests on windows for now.
 
#### Limitations
##### Development
 * Only `init-osx` is provided to help developer to get started. Other platforms are out of scope. It would be possible to install them only locally, using the [installation steps provided here][manual-android-platform-tools]
 * `crystal` currently does not work on windows, which limits development/running the specs to non-windows developers. This choice was made to get rid of Ruby, which was too
 * `make spec` currently needs multiple environment variables to be set, which might be inconvenient to the casual user.
 
##### `Anders` Program Internals
 * Sometimes when doing IO, strings are used as buffers instead of using streams, for convenience.
 * When building program invocations, for convenience formatting functions are used. These enforce usage of UTF8, which can actually cause invalid paths to be generated on filesystems with non-UTF8 paths and non-ascii characters.
 
##### `Anders` Program Features
 * It forces the java source and target language level to 1.7, as `dx` doesn't seem to support a higher level yet.
 * `tasks->(compile|pacakge|launch)->(before|after)` only support what can be specified in `bash -c <cmd-string>`

#### Performance Considerations
 * No caching is done when finding tools in the `PATH` or in the `ANDROID_HOME`, doing more IOPS than strictly needed.
 
#### Security Considerations
Generally, as an unprovileged command-line program, there is probably not too much that can go wrong. However, let's collect all usages of untrusted input that could be used to make the `anders` misbehave.

 * Reads a `.json` file from disk without verifying size or protecting against.
 * Executes programs from the `PATH` and found within `ANDROID_HOME`, and thus trusts them without having any reason to.

[exe-on-windows]: http://stackoverflow.com/questions/37498864/finding-executable-in-path-with-rust
[manual-android-platform-tools]: http://stackoverflow.com/questions/31374085/installing-adb-on-mac-os-x
[meet-crystal]: https://www.youtube.com/watch?v=tAw5puTcGhA
