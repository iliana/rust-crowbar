# rust-crowbar

[![crates.io](https://img.shields.io/crates/v/crowbar.svg)](https://crates.io/crates/crowbar)
[![docs.rs](https://docs.rs/crowbar/badge.svg)](https://docs.rs/crowbar)

crowbar makes it easy to write AWS Lambda functions in Rust. It wraps native Rust functions into CPython modules that handle converting Python objects into Rust objects and back again.

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

## Building Lambda functions

For your code to be usable in AWS Lambda's Python execution environment, you need to compile to a dynamic library with the necessary functions for CPython to run. The `lambda!` macro does most of this for you, but cargo still needs to know what to do.

You can configure cargo to build a dynamic library with the following. If you're using the `lambda!` macro as above, you need to use `lambda` for the library name (see the documentation for `lambda!` if you want to use something else).

```toml
[lib]
name = "lambda"
crate-type = ["cdylib"]
```

`cargo build` will now build a `liblambda.so`. Put this in a zip file and upload it to an AWS Lambda function. Use the Python 3.6 execution environment with the handler configured as `liblambda.handler`.

Because you're building a dynamic library, other libraries that you're dynamically linking against need to also be in the Lambda execution environment. The easiest way to do this is building in an environment similar to Lambda's, such as Amazon Linux. You can use an [EC2 instance](https://aws.amazon.com/amazon-linux-ami/) or a [Docker container](https://hub.docker.com/_/amazonlinux/).

The `builder` directory of the [crowbar git repo](https://github.com/ilianaw/rust-crowbar) contains a `Dockerfile` with Rust set up and a build script to dump a zip file containing a stripped shared library to stdout. Documentation for using that is available at [ilianaw/crowbar-builder on Docker Hub](https://hub.docker.com/r/ilianaw/crowbar-builder/).

## Contributing

crowbar welcomes your contributions:

* Let us know if you use crowbar in production
* If you have a bug report or an idea, [submit an issue](https://github.com/ilianaw/rust-crowbar/issues)
* If you want something to work on, [check the issues list](https://github.com/ilianaw/rust-crowbar/issues)
* Please submit non-trivial changes as an issue first; send a pull request when the implementation is agreed on

crowbar follows a [code of conduct](https://github.com/ilianaw/rust-crowbar/blob/master/CODE_OF_CONDUCT.md); please read it.
