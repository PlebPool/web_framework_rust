extern crate proc_macro;

use proc_macro::TokenStream;
use std::iter::Map;
use quote::quote;
use proc_macros::EnumFromStr;

use syn::*;
use syn::punctuated::Iter;

#[proc_macro_derive(EnumFromStr)]
pub fn string_enum_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let output_token_stream = match data {
        syn::Data::Enum(DataEnum { variants, .. }) => {
            let variant_token_identity_iterator: Map<Iter<Variant>, fn(&Variant) -> &Ident> =
                variants.iter().map(|v| &v.ident);
            let variant_token_identity_string_iterator: Map<Iter<Variant>, fn(&Variant) -> String> =
                variants.iter().map(|v| v.ident.to_string());
            quote! {
                impl #ident {
                    fn from_str(s: &str) -> Result<Self, ()> {
                        match s.to_uppercase().as_str() {
                            #(#variant_token_identity_string_iterator => {
                                Ok( #ident::#variant_token_identity_iterator )
                            },)*
                            _ => {
                                Err(())
                            }
                        }
                    }
                }
            }
        },
        _ => {
            panic!("NOT ENUM")
        }
    };
    output_token_stream.into()
}
