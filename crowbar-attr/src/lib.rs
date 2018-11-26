//! provides function attribute macros for rust-crowbar crate

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{parse, ItemFn, ReturnType};

/// Implements the `lambdafn` attribute.
///
/// This attribute is used to export a Rust function into an
/// AWS triggerable Lambda function. In lambda you can refer to these by path with
/// `liblambda.{fn_name}`
///
/// # Examples
///
/// ```rust,ignore
/// #[macro_use] extern crate crowbar;
/// #[macro_use] extern crate cpython;
///
/// #[lambdafn]
/// fn example(
///     event: crowbar::Value,
///     ctx: crowbar::LambdaContext
/// ) -> crowbar::LambdaResult {
///     Ok(event)
/// }
/// ```
#[proc_macro_attribute]
pub fn lambdafn(args: TokenStream, input: TokenStream) -> TokenStream {
    attr_impl(args, input)
}

// implementation. should expect the following
// * verify function type
// * input args are (event, context)
// * has a return type
fn attr_impl(_: TokenStream, input: TokenStream) -> TokenStream {
    let target: ItemFn = match parse(input.clone()) {
        Ok(f) => f,
        _ => {
            panic!("the 'lambdafn' attribute can only be used on functions");
            // https://doc.rust-lang.org/proc_macro/struct.Span.html#method.error
            // use span diagnotics when this becomes stable
        }
    };
    if target.decl.inputs.len() != 2 {
        panic!(
            "the 'lambdafn' attribute requires a function with two arguments. expecting {}(_: crowbar::Value, _: crowbar::LambdaContext) -> crowbar::LambdaResult", target.ident
            );
            // https://doc.rust-lang.org/proc_macro/struct.Span.html#method.error
            // use span diagnotics when it becomes stable
    }
    match target.decl.output {
        ReturnType::Default => {
            // https://doc.rust-lang.org/proc_macro/struct.Span.html#method.error
            // use span diagnotics when it becomes stable
            panic!("the 'lambdafn' attribute requires a function that returns a value. expecting {}(_: crowbar::Value, _: crowbar::LambdaContext) -> crowbar::LambdaResult", target.ident);
        },
        _ => ()
    }
    let target_ident = target.ident.clone();
    let target_name = target_ident.to_string();
    let expanded = quote! {
        #target

        lambda!(#target_name => #target_ident);
    };
    expanded.into()
}