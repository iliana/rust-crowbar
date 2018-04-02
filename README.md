# rust-crowbar

[![Build Status][travis.svg]][travis]
[![crates.io](https://img.shields.io/crates/v/crowbar.svg)](https://crates.io/crates/crowbar)
[![docs.rs](https://docs.rs/crowbar/badge.svg)](https://docs.rs/crowbar)

crowbar makes it easy to write AWS Lambda functions in Rust. It wraps native Rust functions into CPython modules that
handle converting Python objects into Rust objects and back again.

## Usage

Add both crowbar and cpython to your `Cargo.toml`:

```toml
[dependencies]
crowbar = "0.2"
cpython = "0.1"
```

Use macros from both crates:

```rust
#[macro_use(lambda)]
extern crate crowbar;
#[macro_use]
extern crate cpython;
```

And write your function using the `lambda!` macro:

```rust
lambda!(|event, context| {
    println!("hi cloudwatch logs, this is {}", context.function_name());
    // return the event without doing anything with it
    Ok(event)
});
```

## Building Lambda Functions

For your code to be usable in AWS Lambda's Python execution environment, you need to compile to a dynamic library with
the necessary functions for CPython to run. The `lambda!` macro does most of this for you, but Cargo still needs to know
what to do.

You can configure Cargo to build a dynamic library with the following. If you're using the `lambda!` macro as above, you
need to use `lambda` for the library name (see the documentation for `lambda!` if you want to use something else).

```toml
[lib]
name = "lambda"
crate-type = ["cdylib"]
```

`cargo build` will now build a `liblambda.so`. Put this in a zip file and upload it to an AWS Lambda function. Use the
Python 3.6 execution environment with the handler configured as `liblambda.handler`.

## Build Environment

[It is notoriously difficult to build a properly-linked shared library against the Lambda execution environment.][woes]
Using either an Amazon Linux AMI or the Amazon Linux Docker image, _even if using the exact version as Lambda_, any
package installations will likely upgrade OpenSSL from 1.0.1k to 1.0.2k, causing a linker exception at runtime.
[@naftulikay][naftulikay] lost a ridiculous amount of time trying to statically compile, pin package versions, and
try linker hacks.

The best solution available is to use the [`lambci/lambda:build-python3.6`][lambci/lambda] Docker image which is built
from an exact filesystem replica via tarballing the filesystem at runtime from a Python 3.6 runtime Lambda function.
The authors went to extensive lengths to pin packages and replicate the environment as accurately as possible, and
experience has shown this is the best way to build Python 3.6 shared libraries.

[@naftulikay][naftulikay] created a sample Rust build environment based on the upstream
[`lambci/lambda:build-python3.6`][lambci/lambda] image at
[`naftulikay/circleci-lambda-rust`][naftulikay/circleci-lambda-rust]. Previously,
[`naftulikay/circleci-amazonlinux-rust`][naftulikay/circleci-amazonlinux-rust] was used and the aforementioned issues
were encountered. Despite CircleCI being used in the name, the image is a fairly generic Rust build environment and
should be fairly portable and resuable. For Travis CI and CircleCI examples, please look in the `examples/ci` directory.

Because you're building a dynamic library, other libraries that you're dynamically linking against need to also be in
the Lambda execution environment. By using the [`lambci/lambda:build-python3.6`][lambci/lambda] image, the build
environment will be consistent with the runtime environment.

[As described here][lambda-execution-environment], the Lambda execution environment uses a runtime library path
equivalent to:

```shell
LD_LIBRARY_PATH=/lib64:/usr/lib64:$LAMBDA_TASK_ROOT:$LAMBDA_TASK_ROOT/lib
```

[@naftulikay][naftulikay] wrote a fairly naïve Python script which will recursively copy linked libraries into the
deployment package under `lib/`. This ensures that any non-standard libraries will be available on the library path at
runtime. See the `examples/ci/{travis,circle}` directories for examples on how to use this.

The `builder` directory of the [crowbar git repo](https://github.com/ilianaw/rust-crowbar) contains a `Dockerfile` with
Rust set up and a build script to dump a zip file containing a stripped shared library to stdout. Documentation for
using that is available at [ilianaw/crowbar-builder on Docker Hub](https://hub.docker.com/r/ilianaw/crowbar-builder/).

## Contributing

crowbar welcomes your contributions:

* Let us know if you use crowbar in production
* If you have a bug report or an idea, [submit an issue](https://github.com/ilianaw/rust-crowbar/issues)
* If you want something to work on, [check the issues list](https://github.com/ilianaw/rust-crowbar/issues)
* Please submit non-trivial changes as an issue first; send a pull request when the implementation is agreed on

crowbar follows a [code of conduct](https://github.com/ilianaw/rust-crowbar/blob/master/CODE_OF_CONDUCT.md);
please read it.

 [travis]: https://travis-ci.org/ilianaw/rust-crowbar
 [travis.svg]: https://travis-ci.org/ilianaw/rust-crowbar.svg?branch=master
 [lambci/lambda]: build-python3.6
 [lambda-execution-environment]: https://docs.aws.amazon.com/lambda/latest/dg/current-supported-versions.html
 [naftulikay]: https://github.com/naftulikay
 [naftulikay/circleci-amazonlinux-rust]: https://github.com/naftulikay/docker-circleci-amazonlinux-rust
 [naftulikay/circleci-lambda-rust]: https://github.com/naftulikay/docker-circleci-lambda-rust
 [woes]: https://github.com/naftulikay/docker-circleci-lambda-rust#background
