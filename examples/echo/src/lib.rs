#[macro_use]
extern crate cpython;
#[macro_use(lambda)]
extern crate crowbar;

lambda!(|event, context| {
    println!(
        "hello cloudwatch logs from {} version {}, {} ms remaining",
        context.function_name(),
        context.function_version(),
        context.get_remaining_time_in_millis()?
    );
    Ok(event)
});
