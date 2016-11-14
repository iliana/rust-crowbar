#[macro_use(lambda)]
extern crate crowbar;
#[macro_use]
extern crate cpython;

use crowbar::{PyResult, Value};

fn echo(event: Value) -> PyResult<Value> {
    Ok(event)
}

lambda!(echo);
