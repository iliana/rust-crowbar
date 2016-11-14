#[macro_use(lambda)]
extern crate crowbar;
#[macro_use]
extern crate cpython;

use crowbar::{Value, LambdaResult};

fn echo(event: Value) -> LambdaResult {
    Ok(event)
}

lambda!(echo);
