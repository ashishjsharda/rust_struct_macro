extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute (route string)
    let route = attr.to_string();

    // Parse the function itself
    let input = parse_macro_input!(item as ItemFn);

    // Get the function name and function body
    let fn_name = &input.sig.ident;
    let fn_body = &input.block;

    // Generate the new function with a route message
    let expanded = quote! {
        fn #fn_name() {
            println!("Route: {}", #route);
            #fn_body
        }
    };

    TokenStream::from(expanded)
}
