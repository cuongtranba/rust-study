use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloWorld)]
pub fn hello_world_derive(input: TokenStream) -> TokenStream {
    // Parse input thành syntax tree
    let ast = syn::parse(input).unwrap();

    // Implement trait
    impl_hello_world(&ast)
}

fn impl_hello_world(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloWorld for #name {
            fn hello_world() {
                println!("Hello, World! Tôi là {}", stringify!(#name));
            }
        }
    };
    gen.into()
}
