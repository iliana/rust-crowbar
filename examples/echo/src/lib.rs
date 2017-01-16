#[macro_use(lambda)]
extern crate crowbar;
#[macro_use]
extern crate cpython;

use crowbar::{Value, LambdaContext, LambdaResult};

fn echo(event: Value, context: LambdaContext) -> LambdaResult {
    println!("hello cloudwatch logs from {} version {}, {} ms remaining",
             context.function_name(),
             context.function_version(),
             context.get_remaining_time_in_millis()?);
    Ok(event)
}

lambda!(echo);
