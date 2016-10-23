`and` (pronounced `anders`) is a tool to help with the typical android project development process, and essentially is a convenience wrapper around the lower level plumbing provided by the Android-SDK.

#### Project Goals
* Fulfill uber technical assignment acceptance criteria
* run on all platforms as the Android-SDK
* be sufficiently easy to maintain and extend

#### Project Non-Goals
* be constrained in any way
* Be easy to build on plain Windows due to the use of standard linux tools like `make` and `bash`.

#### Choice of tools
* `make`
 + The project hub. It's the go-to tool to keep track of dependencies between other tools and files to aid the developer.
* `bash`
 + Makes it easy to orchestrate processes and to automate the project. Very efficient in conjunction with make.
* `Rust`
 + Safe programming language that makes it easy to seed most common bugs out at compile-time. Makes it easy to write high-quality, highly maintainable software.