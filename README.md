# rust-crowbar

[![crates.io](https://img.shields.io/crates/v/crowbar.svg)](https://crates.io/crates/crowbar)
[![docs.rs](https://docs.rs/crowbar/badge.svg)](https://docs.rs/crowbar)

crowbar makes it easy to write AWS Lambda functions in Rust. It wraps native Rust functions into CPython modules that handle converting Python objects into Rust objects and back again.

## Usage

Add both crowbar and cpython to your `Cargo.toml`:

```toml
[dependencies]
crowbar = "0.1"
cpython = { version = "*", default-features = false, features = ["python27-sys"] }
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

You can configure cargo to build a dynamic library with the following. Note that the library name *must* be `lambda`.

```toml
[lib]
name = "lambda"
crate-type = ["cdylib"]
```

`cargo build` will now build a `liblambda.so`. Put this in a zip file and upload it to an AWS Lambda function. You will need to use the Python 2.7 execution environment with the handler configured as `liblambda.handler`.

For best results, it's important to build the shared library on a system using the same libraries as the Lambda execution environment. Since Lambda uses Amazon Linux, the easiest way to do this is to use an [EC2 instance](https://aws.amazon.com/amazon-linux-ami/) or a [Docker container](https://hub.docker.com/_/amazonlinux/).

The `builder` directory of the [crowbar git repo](https://github.com/ilianaw/rust-crowbar) contains a `Dockerfile` with Rust set up and a build script to dump a zip file containing a stripped shared library to stdout. Documentation for that is available at [ilianaw/crowbar-builder on Docker Hub](https://hub.docker.com/r/ilianaw/crowbar-builder/).
