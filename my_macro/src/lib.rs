extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(MyTrait)]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_my_trait(&ast)
}

fn impl_my_trait(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl MyTrait for #name {
            fn my_function(&self) {
                println!("Hello, I am {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
