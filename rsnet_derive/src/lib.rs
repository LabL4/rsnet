
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod renderer;

use proc_macro::TokenStream;
use syn::{parse::Parse, Expr, Token};

// use rs

#[proc_macro_derive(Widget)]
pub fn derive(input: TokenStream) -> TokenStream {

    let item = syn::parse(input).expect("failed to parse input");

    match item {
        syn::Item::Struct(ref struct_item) => {
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


struct GetOrReturnNoneMacroInput {
    var_name: syn::Ident,
    _comma: Token![,],
    get_expr: Expr,
    // comma: Token![,],
    // key_expr: Expr
}

impl Parse for GetOrReturnNoneMacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            var_name: input.parse()?,
            _comma: input.parse()?,
            get_expr: input.parse()?,
            // comma: input.parse()?,
            // key_expr: input.parse()?,
        })
    }
}

// Proc macro to get from hashmap or return
#[proc_macro]
pub fn unwrap_or_return_none(input: TokenStream) -> TokenStream {
    let input_expr = syn::parse_macro_input!(input as GetOrReturnNoneMacroInput);


    let var_name = input_expr.var_name;
    let get_expr = input_expr.get_expr;
    

    let output = quote! {
        let #var_name: Option<_> = #get_expr;
        if #var_name.is_none() {
            return None
        }
        let #var_name = #var_name.unwrap();
    };
    output.into()
}

#[proc_macro]
pub fn include_shader(input: TokenStream) -> TokenStream {
    renderer::include_shader(input)
}
