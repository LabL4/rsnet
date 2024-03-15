extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::Item;

// use rs

#[proc_macro_derive(Widget)]
pub fn derive(input: TokenStream) -> TokenStream {

    let item: syn::Item = syn::parse(input).expect("failed to parse input");

    match item {
        Item::Struct(ref struct_item) => {
            let struct_name = &struct_item.ident;
            let expanded = quote! {
                use std::any::Any;
                use crate::gui::state::{AsAny, Widget};

                impl AsAny for #struct_name {
                    fn as_any(&self) -> &dyn Any {
                        self
                    }
        
                    fn as_any_mut(&mut self) -> &mut dyn Any {
                        self
                    }
                }

                impl Widget for #struct_name {}
            };

            return expanded.into();
        },
        _ => {
            // syn::Error::new_spanned(&input, "This is not a struct")
            //     .to_compile_error()
            //     .into()
        },
    };

    let output = quote!{ #item };
    output.into()
}