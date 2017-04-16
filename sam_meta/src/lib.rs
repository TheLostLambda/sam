extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::*;

#[proc_macro_derive(ToForm)]
pub fn to_form(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).unwrap();
    impl_to_form(&ast).parse().unwrap()
}

fn impl_to_form(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let fields = match &ast.body {
        &Body::Struct(VariantData::Struct(ref v)) => v,
        _ => panic!("Attempted derive on a type without named fields!")
    };
    let form: Vec<Input> = Vec::new();
    /*for Field{ident: i, vis: _, attrs: _, ty: t} in fields {
        //Push the input types here. Find a way to splice in the names
    }*/
    quote! {
        impl ToForm for #name {
            fn to_form(names: Vec<&str>) -> Form {
                #form
            }
        }
    }
}
