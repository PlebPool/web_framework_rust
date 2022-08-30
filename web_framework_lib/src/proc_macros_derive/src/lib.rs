extern crate proc_macro;

use proc_macro::TokenStream;
use std::fs;
use std::iter::Map;
use std::str::Split;
use quote::quote;

use syn::*;
use syn::punctuated::Iter;

#[proc_macro_derive(MimeTypeFromEnum)]
pub fn mime_type_from_enum_derive(input: TokenStream) -> TokenStream {
    impl_mime_type(parse_macro_input!(input))
}

#[proc_macro_derive(EnumFromStr)]
pub fn string_enum_derive(input: TokenStream) -> TokenStream {
    impl_enum_from_str(parse_macro_input!(input))
}

/// It reads a file, splits it by newlines, splits each line by semicolons, and then creates a match
/// statement that returns the second part of the split line if the first part matches the enum variant
///
/// Arguments:
///
/// * `derive_input`: DeriveInput
///
/// Returns:
///
/// A TokenStream.
fn impl_mime_type(derive_input: DeriveInput) -> TokenStream {
    let DeriveInput { ident, .. } = derive_input;
    let arr: Vec<u8> = fs::read("src/proc_macros_derive/resource/mime-types.csv")
        .expect("UwU");
    let str: &str = &*String::from_utf8_lossy(&arr);
    let newline_split: Split<char> = str.split('\n');
    let mut keys: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut vals: Vec<String> = Vec::new();
    for line in newline_split {
        println!("MimeType: {}", line);
        let separator_split: (&str, &str) = line.split_once(';').unwrap();
        keys.push(separator_split.0.replace('.', "").to_uppercase().parse()
            .unwrap());
        vals.push(separator_split.1.replace("\r\n", "")
            .replace("\r", ""));
    }
    let quote: proc_macro2::TokenStream = quote! {
        impl #ident {
            pub fn mime_type(&self) -> Result<&str, ()> {
                match self { #(#ident::#keys => { Ok( #vals ) },)*_ => { Err(()) } }
            }
        }
    };
    println!("Generated match stmt:\r\n{}", quote);
    TokenStream::from(quote)
}

/// It takes a `syn::DeriveInput` and returns a `TokenStream` that implements `FromStr` for the enum
///
/// Arguments:
///
/// * `derive_input`: DeriveInput
///
/// Returns:
///
/// A TokenStream
fn impl_enum_from_str(derive_input: DeriveInput) -> TokenStream {
    let DeriveInput { ident, data, .. } = derive_input;
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
